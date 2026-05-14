//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 891/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk891<F: Float>(t8513: F, t9310: F, t9312: F, t9316: F, t9319: F, t9322: F, t8523: F, t8527: F, t8529: F, t10285: F, t10286: F, t37108: F, t7383: F, t7391: F, t9333: F, t8543: F) -> (F, F, F, F, F, F) {
    let t42418 = 0.1702583995731913576e-4 * t8513;
    let t42420 = 0.4726e1 * t9310;
    let t42421 = 0.11974241701863808564e0 * t9312;
    let t42424 = 0.23948483403727617128e0 * t9316;
    let t42425 = 0.35922725105591425692e0 * t9319;
    let t42426 = 0.23948483403727617128e0 * t9322;
    let t42427 = 0.40911992481368012596e-1 * t8523;
    let t42428 = 0.40911992481368012596e-1 * t8527;
    let t42429 = 0.5454932330849068346e-1 * t8529;
    let t42431 = 0.31931311204970156171e0 * t7383 - t42424 + t42425 + t42426 + t42427 + t42428 - t42429 + t10285 - t10286 - t37108 + 0.17347588262831798123e-3 * t7391;
    let t42434 = 0.11974241701863808564e0 * t9333;
    let t42435 = 0.11974241701863808564e0 * t8543;
    (t42418, t42420, t42421, t42431, t42434, t42435)
}
