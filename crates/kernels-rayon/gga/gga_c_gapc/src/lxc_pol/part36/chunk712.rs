//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 712/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk712(t5: f64, t8448: f64, t116: f64, t4048: f64, t134: f64, t667: f64, t5589: f64, t674: f64, t2945: f64, t8316: f64, t2902: f64, t2910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8449 = t5 * t8448;
    let t8450 = t116 * t4048;
    let t8451 = t8449 * t8450;
    let t8452 = t667 * t134;
    let t8454 = t8452 * t674 * t5589;
    let t8455 = t8451 * t8454;
    let t8457 = t8316 * t2945;
    let t8459 = t2902 * t2910;
    (t8449, t8450, t8451, t8452, t8455, t8457, t8459)
}
