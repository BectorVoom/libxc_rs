//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 657/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk657<F: Float>(t2417: F, t4604: F, t119: F, t121: F, t803: F, t120: F, t2250: F, t623: F, t891: F) -> (F, F, F, F) {
    let t7583 = t2417 * t4604;
    let t7584 = t7583 * t119;
    let t7585 = t121 * t803;
    let t7586 = t120 * t7585;
    let t7589 = t2250 * t623;
    let t7590 = t891 * t7589;
    (t7584, t7586, t7589, t7590)
}
