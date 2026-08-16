//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1125/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1125(t9185: f64, t9191: f64, t9195: f64, t9199: f64, t9202: f64, t9207: f64, t9214: f64, t9219: f64, t9223: f64, t9225: f64, t9229: f64, t9236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44485 = 0.5107751987195740728e-4_f64 * t9185;
    let t44486 = 0.10215503974391481456e-3_f64 * t9191;
    let t44487 = 0.15323255961587222184e-3_f64 * t9195;
    let t44488 = 0.5107751987195740728e-4_f64 * t9199;
    let t44489 = 0.5107751987195740728e-4_f64 * t9202;
    let t44490 = 0.638468998399467591e-4_f64 * t9207;
    let t44492 = 0.3405167991463827152e-4_f64 * t9214;
    let t44493 = 0.5107751987195740728e-4_f64 * t9219;
    let t44494 = 0.212822999466489197e-4_f64 * t9223;
    let t44495 = 0.17961362552795712846e0_f64 * t9225;
    let t44496 = 0.11974241701863808564e0_f64 * t9229;
    let t44498 = 0.1702583995731913576e-4_f64 * t9236;
    (t44485, t44486, t44487, t44488, t44489, t44490, t44492, t44493, t44494, t44495, t44496, t44498)
}
