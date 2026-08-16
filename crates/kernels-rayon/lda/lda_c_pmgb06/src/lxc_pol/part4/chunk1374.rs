//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1374/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1374(t118: f64, t5988: f64, t10840: f64, t10843: f64, t10844: f64, t10847: f64, t10848: f64, t10852: f64, t10853: f64, t10855: f64, t10857: f64, t10860: f64, t10861: f64, t10864: f64, t10866: f64, t10867: f64, t10869: f64, t10876: f64) -> f64 {
    let t18054 = t5988 * t118;
    let t18056 = t10840 - t10843 - 0.0002373061974330281_f64 * t10844 - t10847 - 0.06301081444628223_f64 * t10848 - t10852 - 0.031505407223141116_f64 * t10853 + 0.06301081444628223_f64 * t10855 + 0.3780648866776934_f64 * t10857 + t10860 + 0.1890324433388467_f64 * t10861 - t10864 + t10866 - 0.06301081444628223_f64 * t10867 - 0.3780648866776934_f64 * t10869 + t10876 - 0.06301081444628223_f64 * t18054;
    t18056
}
