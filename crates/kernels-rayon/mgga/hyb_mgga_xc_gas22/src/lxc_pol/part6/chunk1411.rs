//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1411/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1411(t259: f64, t30477: f64, t30501: f64, t22102: f64, t22105: f64, t22107: f64, t22112: f64, t22115: f64, t22116: f64, t22120: f64, t22123: f64, t22126: f64, t22127: f64, t22131: f64, t22132: f64, t22134: f64, t22138: f64, t26020: f64, t26023: f64, t29533: f64, t493: f64) -> (f64, f64) {
    let t30503 = (t30477 + t30501) * t259;
    let t30512 = -0.23392894490538584828e1_f64 * t26020 - 0.11696447245269292414e1_f64 * t26023 + 0.19751673498613801407e-1_f64 * t30503 * t493 - 24.0_f64 * t22102 + t22105 + 0.10843581300301739842e-1_f64 * t22107 + t22112 - t22115 - 0.65061487801810439052e-1_f64 * t22116 + t22120 - t22123 - t22126 + 0.32530743900905219526e-1_f64 * t22127 + t22131 + 0.96319466275353142156e0_f64 * t22132 - 0.43374325201206959367e-1_f64 * t22134 + t22138 - t29533;
    (t30503, t30512)
}
