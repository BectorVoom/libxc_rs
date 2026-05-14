//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 888/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk888<F: Float>(t1447: F, t6752: F, t187: F, t7209: F, t7179: F, t161: F, t489: F, t6595: F, t1916: F, t5194: F, t1920: F, t2497: F, t3223: F, t1887: F, t1928: F, t4810: F, t802: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17809 = t1447 * t6752;
    let t17859 = t7209 * t187;
    let t17861 = t7179 * t187;
    let t17875 = t161 * t489 * t6595;
    let t17886 = t5194 * t1916;
    let t17890 = t5194 * t1920;
    let t17909 = t3223 * t2497;
    let t17919 = t1887 * t1928;
    let t17921 = t802 * t4810;
    (t17809, t17859, t17861, t17875, t17886, t17890, t17909, t17919, t17921)
}
