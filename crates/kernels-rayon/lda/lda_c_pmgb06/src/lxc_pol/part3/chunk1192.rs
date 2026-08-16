//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1192/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1192(t11837: f64, t11842: f64, t11843: f64, t11845: f64, t11846: f64, t11847: f64, t11853: f64, t11859: f64, t11861: f64, t11865: f64, t11867: f64, t11869: f64, t11872: f64, t11874: f64, t11876: f64, t11880: f64, t11882: f64, t11885: f64, t11886: f64, t11889: f64, t11890: f64, t11891: f64, t11892: f64) -> (f64, f64) {
    let t14325 = -t11837 - t11842 + t11843 - t11845 + t11846 - t11847 + t11853 + t11859 - t11861 + t11865 - t11867;
    let t14326 = t11869 - t11872 - t11874 - t11876 + t11880 + t11882 + t11885 + t11886 + t11889 - t11890 - t11891 - t11892;
    (t14325, t14326)
}
