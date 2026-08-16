//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1854/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1854(t1873: f64, t26114: f64, t4072: f64, t88: f64, t6534: f64, t7676: f64, t2314: f64, t7467: f64, t5113: f64, t1453: f64, t22470: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26116 = 2.0_f64 * t26114 * t1873;
    let t26117 = t88 * t4072;
    let t26119 = 2.0_f64 * t26117 * t1873;
    let t26121 = 2.0_f64 * t7676 * t6534;
    let t26123 = 2.0_f64 * t2314 * t7467;
    let t26125 = 2.0_f64 * t5113 * t7467;
    let t26127 = t22470 * t1453;
    let t26129 = t1453 * t666;
    (t26116, t26117, t26119, t26121, t26123, t26125, t26127, t26129)
}
