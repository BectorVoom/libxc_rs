//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1411/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1411<F: Float>(t259: F, t30477: F, t30501: F, t22102: F, t22105: F, t22107: F, t22112: F, t22115: F, t22116: F, t22120: F, t22123: F, t22126: F, t22127: F, t22131: F, t22132: F, t22134: F, t22138: F, t26020: F, t26023: F, t29533: F, t493: F) -> (F, F) {
    let t30503 = (t30477 + t30501) * t259;
    let t30512 = -F::new(0.23392894490538584828e1) * t26020 - F::new(0.11696447245269292414e1) * t26023 + F::new(0.19751673498613801407e-1) * t30503 * t493 - F::new(24.0) * t22102 + t22105 + F::new(0.10843581300301739842e-1) * t22107 + t22112 - t22115 - F::new(0.65061487801810439052e-1) * t22116 + t22120 - t22123 - t22126 + F::new(0.32530743900905219526e-1) * t22127 + t22131 + F::new(0.96319466275353142156e0) * t22132 - F::new(0.43374325201206959367e-1) * t22134 + t22138 - t29533;
    (t30503, t30512)
}
