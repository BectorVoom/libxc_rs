//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1121/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1121(t38031: f64, t43868: f64, t43869: f64, t47541: f64, t47545: f64, t47549: f64, t47553: f64, t47557: f64, t47561: f64, t47565: f64, t47570: f64, t47572: f64, t47577: f64, t47581: f64, t47585: f64, t47588: f64, t5267: f64, t5928: f64, t884: f64, t9530: f64, t9627: f64) -> f64 {
    let t49256 = -0.19863479950205658386e-4_f64 * t47541 + t43868 + t43869 + 0.1440846329149835838e-2_f64 * t47545 + 0.72042316457491791901e-3_f64 * t47549 + 0.1440846329149835838e-2_f64 * t47553 + t38031 - 0.15323255961587222184e-3_f64 * t47557 - 0.5107751987195740728e-4_f64 * t47561 + 0.10215503974391481456e-3_f64 * t47565 - 0.1702583995731913576e-4_f64 * t47570 - 0.1702583995731913576e-4_f64 * t47572 - 0.2553875993597870364e-4_f64 * t47577 + 0.5107751987195740728e-4_f64 * t47581 - 0.7661627980793611092e-4_f64 * t47585 - 0.5987120850931904282e-1_f64 * t47588 - 0.23948483403727617128e0_f64 * t884 * t9530 * t5267 - 0.23948483403727617128e0_f64 * t5928 * t9627;
    t49256
}
