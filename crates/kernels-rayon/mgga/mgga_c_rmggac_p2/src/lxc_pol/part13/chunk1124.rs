//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1124/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1124(t10508: f64, t44470: f64, t44472: f64, t44473: f64, t44474: f64, t44475: f64, t44476: f64, t44477: f64, t44478: f64, t44479: f64, t9133: f64, t9659: f64) -> (f64, f64) {
    let t44480 = -t44470 - 0.40911992481368012596e-1_f64 * t9133 + t44472 + t10508 + t44473 - t44474 + t44475 - t44476 + t44477 + t44478 + t44479;
    let t44482 = 2.0_f64 * t9659;
    (t44480, t44482)
}
