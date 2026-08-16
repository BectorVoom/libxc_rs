//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 879/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk879(t9154: f64, t9160: f64, t9166: f64, t9172: f64, t9659: f64, t9185: f64, t9191: f64, t9195: f64, t9199: f64, t9202: f64, t9207: f64, t9214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44476 = 0.5107751987195740728e-4_f64 * t9154;
    let t44477 = 0.5107751987195740728e-4_f64 * t9160;
    let t44478 = 0.1702583995731913576e-4_f64 * t9166;
    let t44479 = 0.1702583995731913576e-4_f64 * t9172;
    let t44482 = 2.0_f64 * t9659;
    let t44485 = 0.5107751987195740728e-4_f64 * t9185;
    let t44486 = 0.10215503974391481456e-3_f64 * t9191;
    let t44487 = 0.15323255961587222184e-3_f64 * t9195;
    let t44488 = 0.5107751987195740728e-4_f64 * t9199;
    let t44489 = 0.5107751987195740728e-4_f64 * t9202;
    let t44490 = 0.638468998399467591e-4_f64 * t9207;
    let t44492 = 0.3405167991463827152e-4_f64 * t9214;
    (t44476, t44477, t44478, t44479, t44482, t44485, t44486, t44487, t44488, t44489, t44490, t44492)
}
