//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 848/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk848<F: Float>(t1729: F, t452: F, t454: F, t1872: F, t2765: F, t1184: F, t780: F, t483: F, t1187: F, t169: F, t1891: F, t301: F, t717: F) -> (F, F, F, F, F, F) {
    let t5924 = t1729 * t452 * t454;
    let t5925 = t2765 * t1872;
    let t5931 = t1184 * t780;
    let t5932 = t5931 * t483;
    let t5933 = t5932 * t1187;
    let t5941 = F::new(0.10809180959278285) * t169 * t717 * t1891 * t301;
    (t5924, t5925, t5931, t5932, t5933, t5941)
}
