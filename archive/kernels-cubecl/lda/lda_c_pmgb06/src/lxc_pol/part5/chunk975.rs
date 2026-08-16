//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 975/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk975<F: Float>(t132: F, t435: F, t6674: F, t1447: F, t6114: F, t1995: F, t5194: F, t464: F, t6673: F, t6599: F, t432: F, t6613: F) -> (F, F, F, F, F, F) {
    let t16241 = t132 * t435 * t6674;
    let t16249 = t1447 * t6114;
    let t16254 = t5194 * t1995;
    let t16267 = t6673 * t464;
    let t16284 = t132 * t435 * t6599;
    let t16286 = t432 * t6613;
    (t16241, t16249, t16254, t16267, t16284, t16286)
}
