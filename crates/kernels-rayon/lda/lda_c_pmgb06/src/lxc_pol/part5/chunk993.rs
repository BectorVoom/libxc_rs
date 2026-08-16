//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 993/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk993(t1447: f64, t6752: f64, t187: f64, t7209: f64, t7179: f64, t161: f64, t489: f64, t6595: f64, t1916: f64, t5194: f64, t1920: f64, t2497: f64, t3223: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17809 = t1447 * t6752;
    let t17859 = t7209 * t187;
    let t17861 = t7179 * t187;
    let t17875 = t161 * t489 * t6595;
    let t17886 = t5194 * t1916;
    let t17890 = t5194 * t1920;
    let t17909 = t3223 * t2497;
    (t17809, t17859, t17861, t17875, t17886, t17890, t17909)
}
