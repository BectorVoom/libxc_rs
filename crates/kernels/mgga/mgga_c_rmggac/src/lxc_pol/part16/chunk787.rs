//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 787/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk787<F: Float>(t9126: F, t9129: F, t9135: F, t9139: F, t9143: F, t9148: F, t9154: F, t9160: F, t9166: F, t9172: F, t9659: F, t9185: F, t9191: F, t9195: F, t9199: F, t9202: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t44468 = 0.5987120850931904282e-1 * t9126;
    let t44470 = 0.11974241701863808564e0 * t9129;
    let t44472 = 0.5454932330849068346e-1 * t9135;
    let t44473 = 0.3405167991463827152e-4 * t9139;
    let t44474 = 0.1702583995731913576e-4 * t9143;
    let t44475 = 0.212822999466489197e-4 * t9148;
    let t44476 = 0.5107751987195740728e-4 * t9154;
    let t44477 = 0.5107751987195740728e-4 * t9160;
    let t44478 = 0.1702583995731913576e-4 * t9166;
    let t44479 = 0.1702583995731913576e-4 * t9172;
    let t44482 = 2.0 * t9659;
    let t44485 = 0.5107751987195740728e-4 * t9185;
    let t44486 = 0.10215503974391481456e-3 * t9191;
    let t44487 = 0.15323255961587222184e-3 * t9195;
    let t44488 = 0.5107751987195740728e-4 * t9199;
    let t44489 = 0.5107751987195740728e-4 * t9202;
    (t44468, t44470, t44472, t44473, t44474, t44475, t44476, t44477, t44478, t44479, t44482, t44485, t44486, t44487, t44488, t44489)
}
