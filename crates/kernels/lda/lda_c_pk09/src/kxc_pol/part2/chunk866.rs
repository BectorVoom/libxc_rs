//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 866/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk866<F: Float>(t2259: F, t3104: F, t119: F, t7693: F, t2336: F, t1098: F, t8092: F, t2152: F, t4086: F, t891: F, t3743: F, t8392: F) -> (F, F, F, F, F, F) {
    let t8973 = t3104 * t2259;
    let t8975 = t119 * t7693;
    let t8977 = t2336 * t119;
    let t8980 = t1098 * t8092;
    let t8987 = t891 * t4086 * t2152;
    let t8990 = t8392 * t3743;
    (t8973, t8975, t8977, t8980, t8987, t8990)
}
