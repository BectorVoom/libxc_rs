//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1048/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1048<F: Float>(t27: F, t693: F, t7402: F, t8545: F, t8548: F, t8552: F, t8553: F, t8555: F, t8559: F, t8560: F, t8564: F, t8567: F, t8570: F, t8576: F, t8580: F, t8583: F, t8586: F, t8589: F, t8594: F) -> (F,) {
    let t21727 = t7402 * t27 * t693;
    let t21729 = -12.0 * t8545 + t8548 - t8552 + 0.01626537195045261 * t8553 - 0.03253074390090522 * t8555 - t8559 - 0.02168716260060348 * t8560 - t8564 - t8567 + t8570 + 0.4815973313767657 * t8576 + t8580 + t8583 + t8586 + t8589 - t8594 - 0.00018311447306006544 * t21727;
    (t21729,)
}
