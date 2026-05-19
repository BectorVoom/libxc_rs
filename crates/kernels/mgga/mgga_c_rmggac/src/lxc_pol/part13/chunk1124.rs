//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1124/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1124<F: Float>(t10508: F, t44470: F, t44472: F, t44473: F, t44474: F, t44475: F, t44476: F, t44477: F, t44478: F, t44479: F, t9133: F, t9659: F) -> (F, F) {
    let t44480 = -t44470 - F::cast_from(0.40911992481368012596e-1_f64) * t9133 + t44472 + t10508 + t44473 - t44474 + t44475 - t44476 + t44477 + t44478 + t44479;
    let t44482 = F::new(2.0) * t9659;
    (t44480, t44482)
}
