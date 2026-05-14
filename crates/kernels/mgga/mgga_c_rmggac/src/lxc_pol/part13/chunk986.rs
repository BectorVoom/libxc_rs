//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 986/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk986<F: Float>(t9191: F, t9195: F, t9199: F, t9202: F, t9207: F, t9214: F, t9219: F, t9223: F, t9225: F, t9229: F, t9236: F, t9675: F, t9678: F, t8307: F, t8309: F, t8310: F) -> (F, F, F, F, F, F) {
    let t44486 = 0.10215503974391481456e-3 * t9191;
    let t44487 = 0.15323255961587222184e-3 * t9195;
    let t44488 = 0.5107751987195740728e-4 * t9199;
    let t44489 = 0.5107751987195740728e-4 * t9202;
    let t44490 = 0.638468998399467591e-4 * t9207;
    let t44492 = 0.3405167991463827152e-4 * t9214;
    let t44493 = 0.5107751987195740728e-4 * t9219;
    let t44494 = 0.212822999466489197e-4 * t9223;
    let t44495 = 0.17961362552795712846e0 * t9225;
    let t44496 = 0.11974241701863808564e0 * t9229;
    let t44498 = 0.1702583995731913576e-4 * t9236;
    let t44499 = 0.4726e1 * t9675;
    let t44500 = 2.0 * t9678;
    let t44501 = t44492 - t44493 - t44494 + t8307 + t44495 - t44496 + t8309 - 0.9452e1 * t8310 - t44498 - t44499 + t44500;
    (t44486, t44487, t44488, t44489, t44490, t44501)
}
