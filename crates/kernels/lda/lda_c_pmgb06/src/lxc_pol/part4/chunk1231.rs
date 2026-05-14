//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1231/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1231<F: Float>(t17829: F, t17830: F, t17831: F, t17832: F, t17833: F, t17834: F, t17835: F, t17836: F, t17837: F, t17838: F, t17839: F, t17842: F, t17843: F, t17844: F, t17845: F, t17846: F, t17847: F, t17848: F, t17849: F, t17850: F, t17851: F, t17852: F, t17853: F, t17854: F, t17855: F, t17856: F, t17858: F, t17863: F, t17869: F, t17871: F) -> (F, F) {
    let t18385 = -t17829 - t17830 + t17831 + t17832 + t17833 + t17834 + t17835 - t17836 - t17837 - t17838 - t17839 + t17842 - t17843 - t17844 - t17845;
    let t18386 = -t17846 + t17847 - t17848 - t17849 + t17850 - t17851 - t17852 + t17853 + t17854 + t17855 - t17856 - t17858 + t17863 - t17869 - t17871;
    (t18385, t18386)
}
