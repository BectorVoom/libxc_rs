//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1083/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1083(t12864: f64, t1915: f64, t493: f64, t1912: f64, t3223: f64, t1916: f64, t12839: f64, t12844: f64, t12846: f64, t12849: f64, t12852: f64, t12855: f64, t12857: f64, t12859: f64, t12863: f64) -> (f64, f64, f64, f64) {
    let t12867 = 8.0_f64 / 15.0_f64 * t493 * t1915 * t12864;
    let t12868 = t3223 * t1912;
    let t12869 = 2.0_f64 / 135.0_f64 * t12868;
    let t12870 = t3223 * t1916;
    let t12871 = 4.0_f64 / 135.0_f64 * t12870;
    let t12872 = t12839 + t12844 - t12846 - t12849 - t12852 + t12855 - t12857 - t12859 - t12863 - t12867 + t12869 + t12871;
    (t12867, t12869, t12871, t12872)
}
