//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 80/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk80(t143: f64, t147: f64, t151: f64, t154: f64, t157: f64, t160: f64, t163: f64, t166: f64, t169: f64, t172: f64, t187: f64) -> f64 {
    let t144 = 0.135e1_f64 <= t143;
    let t191 = piecewise3(t144, 1.0_f64 / t147 / 36.0_f64 - t151 / 960.0_f64 + t154 / 26880.0_f64 - t157 / 829440.0_f64 + t160 / 28385280.0_f64 - t163 / 0.107347968e10_f64 + t166 / 0.445906944e11_f64 - t169 / 0.20214448128e13_f64, 1.0_f64 - 8.0_f64 / 3.0_f64 * t172 * t187);
    t191
}
