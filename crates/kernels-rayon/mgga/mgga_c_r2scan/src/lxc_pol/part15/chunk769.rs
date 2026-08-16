//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 769/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk769(t2148: f64, t6402: f64, t2147: f64, t1267: f64, t512: f64, t57: f64, t2158: f64, t2155: f64, t5116: f64, t1415: f64, t511: f64, t2162: f64, t2164: f64) -> (f64, f64, f64, f64, f64) {
    let t6403 = t2148 * t6402;
    let t6404 = t2147 * t6403;
    let t6407 = t512 * t1267 * t57;
    let t6408 = t6407 * t2158;
    let t6410 = t2155 * t5116;
    let t6412 = t1415 * t511;
    let t6415 = 0.89443204944342177673e-3_f64 * t6412 * t2162 * t2164;
    (t6404, t6407, t6408, t6410, t6415)
}
