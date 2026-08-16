//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1017/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1017(t10834: f64, t10838: f64, t10840: f64, t10843: f64, t10844: f64, t10847: f64, t10848: f64, t10852: f64, t10857: f64, t10860: f64, t10861: f64, t10864: f64, t10866: f64, t10869: f64, t10876: f64) -> f64 {
    let t19148 = 0.01975389032890948_f64 * t10834 + t10838 + t10840 - t10843 - 0.00011865309871651405_f64 * t10844 - t10847 - 0.031505407223141116_f64 * t10848 - t10852 + 0.1890324433388467_f64 * t10857 + t10860 + 0.09451622166942335_f64 * t10861 - t10864 + t10866 - 0.1890324433388467_f64 * t10869 + t10876;
    t19148
}
