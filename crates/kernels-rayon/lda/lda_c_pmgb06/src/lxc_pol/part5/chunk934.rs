//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 934/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk934(t12831: f64, t1600: f64, t1835: f64, t1912: f64, t3223: f64, t1916: f64, t1920: f64, t1847: f64, t607: f64, t12514: f64, t1461: f64, t5065: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12832 = t12831 / 45.0_f64;
    let t12840 = t1835 * t1600;
    let t12868 = t3223 * t1912;
    let t12869 = 2.0_f64 / 135.0_f64 * t12868;
    let t12870 = t3223 * t1916;
    let t12871 = 4.0_f64 / 135.0_f64 * t12870;
    let t12878 = t3223 * t1920;
    let t12879 = 2.0_f64 / 81.0_f64 * t12878;
    let t12912 = t1847 * t607;
    let t12981 = t5065 * t12514 * t1461;
    (t12832, t12840, t12869, t12871, t12879, t12912, t12981)
}
