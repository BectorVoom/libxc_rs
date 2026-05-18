//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1046/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1046<F: Float>(t40822: F, t40825: F, t40828: F, t40833: F, t43202: F, t43207: F, t43208: F, t43209: F, t43212: F, t43216: F, t43220: F, t43222: F, t43237: F, t47708: F, t47709: F, t47711: F, t47714: F, t47720: F) -> F {
    let t51038 = -t43202 + t47708 + F::new(0.41016139894091862845e-1) * t47709 + F::new(0.30762104920568897134e-1) * t47711 + F::new(0.30762104920568897134e-1) * t47714 + F::new(0.19226315575355560709e-2) * t40822 - F::new(0.38452631150711121418e-2) * t40825 - F::new(0.12817543716903707139e-2) * t40828 + F::new(0.25635087433807414278e-2) * t40833 + t43207 + t43208 + t43209 + t43212 + t43216 + t43220 + t43222 - t43237 - F::new(0.61524209841137794269e-1) * t47720;
    t51038
}
