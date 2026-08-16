//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1089/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1089(t1912: f64, t3223: f64, t1916: f64, t1920: f64, t1179: f64, t161: f64, t4840: f64, t495: f64, t1447: f64, t5180: f64, t1847: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12868 = t3223 * t1912;
    let t12870 = t3223 * t1916;
    let t12878 = t3223 * t1920;
    let t12898 = t161 * t1179 * t495 * t4840;
    let t12908 = t1447 * t5180;
    let t12912 = t1847 * t607;
    (t12868, t12870, t12878, t12898, t12908, t12912)
}
