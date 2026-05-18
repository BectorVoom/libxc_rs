//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1262/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1262<F: Float>(t7016: F, t795: F, t185: F, t514: F, t7793: F, t511: F, t7794: F, t331: F, t7770: F, t7773: F, t20007: F, t504: F) -> (F, F, F, F, F, F) {
    let t22630 = t795 * t7016;
    let t22631 = F::new(4.0) / F::new(15.0) * t22630;
    let t22633 = t185 * t514 * t7793;
    let t22634 = F::new(4.0) / F::new(45.0) * t22633;
    let t22636 = F::new(2.0) / F::new(15.0) * t511 * t7794;
    let t22649 = t331 * t7770;
    let t22651 = t331 * t7773;
    let t22653 = t504 * t20007;
    (t22631, t22634, t22636, t22649, t22651, t22653)
}
