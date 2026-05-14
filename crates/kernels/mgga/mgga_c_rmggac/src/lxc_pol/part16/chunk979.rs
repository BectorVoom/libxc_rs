//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 979/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk979<F: Float>(t38031: F, t43868: F, t43869: F, t47541: F, t47545: F, t47549: F, t47553: F, t47557: F, t47561: F, t47565: F, t47570: F, t47572: F, t47577: F, t47581: F, t47585: F, t47588: F, t5267: F, t5928: F, t884: F, t9530: F, t9627: F) -> (F,) {
    let t49256 = -0.19863479950205658386e-4 * t47541 + t43868 + t43869 + 0.1440846329149835838e-2 * t47545 + 0.72042316457491791901e-3 * t47549 + 0.1440846329149835838e-2 * t47553 + t38031 - 0.15323255961587222184e-3 * t47557 - 0.5107751987195740728e-4 * t47561 + 0.10215503974391481456e-3 * t47565 - 0.1702583995731913576e-4 * t47570 - 0.1702583995731913576e-4 * t47572 - 0.2553875993597870364e-4 * t47577 + 0.5107751987195740728e-4 * t47581 - 0.7661627980793611092e-4 * t47585 - 0.5987120850931904282e-1 * t47588 - 0.23948483403727617128e0 * t884 * t9530 * t5267 - 0.23948483403727617128e0 * t5928 * t9627;
    (t49256,)
}
