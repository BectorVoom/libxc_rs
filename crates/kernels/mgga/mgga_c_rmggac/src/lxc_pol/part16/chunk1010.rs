//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1010/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1010<F: Float>(t38174: F, t44467: F, t44468: F, t44470: F, t44472: F, t44473: F, t44474: F, t44475: F, t44476: F, t9133: F, t9650: F, t44489: F, t44490: F, t44492: F, t44493: F, t44494: F, t44495: F, t44496: F, t44498: F, t44499: F, t44500: F, t8311: F) -> (F, F) {
    let t49894 = t38174 - t44467 + t44468 - t44470 - 0.40911992481368012595e-1 * t9133 + t44472 + 4.0 * t9650 + t44473 - t44474 + t44475 - t44476;
    let t49897 = t44489 + t44490 + t44492 - t44493 - t44494 + t44495 - t44496 - t8311 - t44498 - t44499 + t44500;
    (t49894, t49897)
}
