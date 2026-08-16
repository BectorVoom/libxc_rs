//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 894/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk894(t7691: f64, t8616: f64, t27146: f64, t3351: f64, t3352: f64, t515: f64, t7720: f64, t8587: f64, t34847: f64, t9206: f64, t1001: f64, t236: f64, t615: f64, t7230: f64, t9210: f64) -> (f64, f64, f64, f64, f64) {
    let t39457 = t7691 * t8616;
    let t39461 = t3351 * t3352 * t515 * t27146;
    let t39463 = t7720 * t8587;
    let t39465 = t34847 * t9206;
    let t39470 = t7230 * t9210 * t236 * t615 * t1001;
    (t39457, t39461, t39463, t39465, t39470)
}
