//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 958/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk958(t12270: f64, t2592: f64, t13765: f64, t4342: f64, t1382: f64, t2497: f64, t3718: f64, t40942: f64, t40946: f64, t3720: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47112 = t2592 * t12270;
    let t47114 = t4342 * t13765;
    let t47120 = t1382 * t3718 * t2497;
    let t47126 = 0.15337170381568299871e1_f64 * t40942;
    let t47127 = 0.38342925953920749677e0_f64 * t40946;
    let t47130 = t3720 * t935;
    (t47112, t47114, t47120, t47126, t47127, t47130)
}
