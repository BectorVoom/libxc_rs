//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1374/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1374<F: Float>(t118: F, t5988: F, t10840: F, t10843: F, t10844: F, t10847: F, t10848: F, t10852: F, t10853: F, t10855: F, t10857: F, t10860: F, t10861: F, t10864: F, t10866: F, t10867: F, t10869: F, t10876: F) -> F {
    let t18054 = t5988 * t118;
    let t18056 = t10840 - t10843 - F::cast_from(0.0002373061974330281_f64) * t10844 - t10847 - F::cast_from(0.06301081444628223_f64) * t10848 - t10852 - F::cast_from(0.031505407223141116_f64) * t10853 + F::cast_from(0.06301081444628223_f64) * t10855 + F::cast_from(0.3780648866776934_f64) * t10857 + t10860 + F::cast_from(0.1890324433388467_f64) * t10861 - t10864 + t10866 - F::cast_from(0.06301081444628223_f64) * t10867 - F::cast_from(0.3780648866776934_f64) * t10869 + t10876 - F::cast_from(0.06301081444628223_f64) * t18054;
    t18056
}
