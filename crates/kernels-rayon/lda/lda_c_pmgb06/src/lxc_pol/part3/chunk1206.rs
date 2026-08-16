//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1206/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1206(t12859: f64, t12863: f64, t12867: f64, t12869: f64, t12871: f64, t12879: f64, t12881: f64, t12883: f64, t12885: f64, t12887: f64, t12889: f64, t12892: f64, t12894: f64, t12895: f64, t12899: f64, t12900: f64, t12902: f64, t12907: f64, t12909: f64, t12911: f64, t12914: f64, t12916: f64, t12918: f64) -> (f64, f64) {
    let t14395 = -t12859 - t12863 - t12867 + t12869 + t12871 - t12879 - t12881 - t12883 - t12885 + t12887 - t12889;
    let t14396 = -t12892 - t12894 + t12895 + t12899 + t12900 + t12902 + t12907 + t12909 + t12911 + t12914 + t12916 + t12918;
    (t14395, t14396)
}
