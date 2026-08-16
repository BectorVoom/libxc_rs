//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 656/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk656(t511: f64, t9104: f64, t7231: f64, t3351: f64, t352: f64, t618: f64, t515: f64, t2283: f64, t7720: f64, t236: f64, t495: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9105 = t511 * t9104;
    let t9106 = t7231 * t9105;
    let t9107 = t3351 * t9106;
    let t9109 = t618 * t352;
    let t9110 = t515 * t9109;
    let t9111 = t7231 * t9110;
    let t9112 = t3351 * t9111;
    let t9114 = t7720 * t2283;
    let t9117 = t236 * t551 * t495;
    (t9106, t9107, t9111, t9112, t9114, t9117)
}
