//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1156/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1156(t38174: f64, t44467: f64, t44468: f64, t44470: f64, t44472: f64, t44473: f64, t44474: f64, t44475: f64, t44476: f64, t9133: f64, t9650: f64, t44489: f64, t44490: f64, t44492: f64, t44493: f64, t44494: f64, t44495: f64, t44496: f64, t44498: f64, t44499: f64, t44500: f64, t8311: f64) -> (f64, f64) {
    let t49894 = t38174 - t44467 + t44468 - t44470 - 0.40911992481368012595e-1_f64 * t9133 + t44472 + 4.0_f64 * t9650 + t44473 - t44474 + t44475 - t44476;
    let t49897 = t44489 + t44490 + t44492 - t44493 - t44494 + t44495 - t44496 - t8311 - t44498 - t44499 + t44500;
    (t49894, t49897)
}
