//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1346/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1346(t12055: f64, t4915: f64, t687: f64, t10099: f64, t11155: f64, t1049: f64, t10786: f64, t1616: f64, t10526: f64, t3179: f64, t13281: f64, t1617: f64, t3808: f64) -> (f64, f64, f64, f64, f64) {
    let t36275 = 12.0_f64 * t4915 * t12055 * t687;
    let t36280 = 2.0_f64 * t10099 * t11155;
    let t36283 = 2.0_f64 * t1616 * t10786 * t1049;
    let t36285 = 2.0_f64 * t10526 * t3179;
    let t36288 = 24.0_f64 * t13281 * t3808 * t1617;
    (t36275, t36280, t36283, t36285, t36288)
}
