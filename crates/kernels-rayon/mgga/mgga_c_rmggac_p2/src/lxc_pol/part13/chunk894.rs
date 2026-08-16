//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 894/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk894(t333: f64, t3351: f64, t511: f64, t7248: f64, t9216: f64, t352: f64, t515: f64, t1970: f64, t1971: f64, t236: f64, t5601: f64, t38350: f64, t7473: f64) -> (f64, f64, f64, f64) {
    let t39813 = t3351 * t7248 * t511 * t9216 * t333;
    let t39818 = t3351 * t7248 * t515 * t9216 * t352;
    let t39830 = t1970 * t1971 * t236 * t5601;
    let t39832 = t38350 * t7473;
    (t39813, t39818, t39830, t39832)
}
