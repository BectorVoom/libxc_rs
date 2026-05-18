//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 989/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk989<F: Float>(t5051: F, t802: F, t1548: F, t2592: F, t1447: F, t6770: F, t1887: F, t2015: F, t27: F, t545: F, t7209: F, t7179: F) -> (F, F, F, F, F, F) {
    let t17372 = t802 * t5051;
    let t17374 = t2592 * t1548;
    let t17376 = t1447 * t6770;
    let t17506 = t1887 * t2015;
    let t17544 = t7209 * t27 * t545;
    let t17547 = t7179 * t27 * t545;
    (t17372, t17374, t17376, t17506, t17544, t17547)
}
