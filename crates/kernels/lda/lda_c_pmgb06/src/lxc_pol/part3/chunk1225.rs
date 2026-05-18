//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1225/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1225<F: Float>(t10840: F, t10843: F, t10844: F, t10847: F, t10848: F, t10852: F, t10853: F, t10855: F, t10857: F, t10860: F, t10861: F, t10864: F, t10866: F, t10867: F, t10869: F) -> F {
    let t14521 = t10840 - t10843 - F::new(0.00035595929614954216) * t10844 - t10847 - F::new(0.09451622166942335) * t10848 - t10852 - F::new(0.09451622166942335) * t10853 + F::new(0.1890324433388467) * t10855 + F::new(0.5670973300165402) * t10857 + t10860 + F::new(0.2835486650082701) * t10861 - t10864 + t10866 - F::new(0.1890324433388467) * t10867 - F::new(0.5670973300165402) * t10869;
    t14521
}
