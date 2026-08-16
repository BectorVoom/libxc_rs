//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 879/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk879(t7244: f64, t8497: f64, t7914: f64, t8571: f64, t1243: f64, t236: f64, t3351: f64, t551: f64, t7248: f64, t3352: f64, t511: f64, t5199: f64) -> (f64, f64, f64, f64) {
    let t39264 = t7244 * t8497;
    let t39265 = 0.19863479950205658386e-4_f64 * t39264;
    let t39266 = t8571 * t7914;
    let t39271 = t3351 * t7248 * t236 * t551 * t1243;
    let t39275 = t3351 * t3352 * t511 * t5199;
    (t39265, t39266, t39271, t39275)
}
