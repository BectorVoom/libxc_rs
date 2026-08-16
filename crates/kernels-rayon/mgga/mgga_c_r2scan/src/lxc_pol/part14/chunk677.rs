//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 677/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk677(t1561: f64, t1563: f64, t2259: f64, t498: f64, t1559: f64, t282: f64, t283: f64) -> (f64, f64, f64) {
    let t5078 = t1561 * t1563;
    let t5081 = t498 * t2259;
    let t5084 = t1559 * t282;
    let t5086 = 1.0_f64 / t283 / t5084;
    (t5078, t5081, t5086)
}
