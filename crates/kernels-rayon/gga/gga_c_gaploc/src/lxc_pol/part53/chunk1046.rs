//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1046/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1046(t40822: f64, t40825: f64, t40828: f64, t40833: f64, t43202: f64, t43207: f64, t43208: f64, t43209: f64, t43212: f64, t43216: f64, t43220: f64, t43222: f64, t43237: f64, t47708: f64, t47709: f64, t47711: f64, t47714: f64, t47720: f64) -> f64 {
    let t51038 = -t43202 + t47708 + 0.41016139894091862845e-1_f64 * t47709 + 0.30762104920568897134e-1_f64 * t47711 + 0.30762104920568897134e-1_f64 * t47714 + 0.19226315575355560709e-2_f64 * t40822 - 0.38452631150711121418e-2_f64 * t40825 - 0.12817543716903707139e-2_f64 * t40828 + 0.25635087433807414278e-2_f64 * t40833 + t43207 + t43208 + t43209 + t43212 + t43216 + t43220 + t43222 - t43237 - 0.61524209841137794269e-1_f64 * t47720;
    t51038
}
