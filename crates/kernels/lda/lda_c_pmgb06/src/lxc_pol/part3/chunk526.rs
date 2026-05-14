//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 526/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk526<F: Float>(t118: F, t2813: F, t1329: F, t415: F, t1186: F, t1334: F, t421: F, t2777: F, t2780: F, t2793: F, t2794: F, t2797: F, t2804: F, t2807: F, t2809: F, t2812: F) -> (F, F, F, F) {
    let t2814 = t2813 * t118;
    let t2816 = t1329 * t415;
    let t2820 = 0.01975389032890948 * t1334 * t1186 * t421;
    let t2821 = t2777 - t2780 - t2793 - 0.09451622166942335 * t2794 + t2797 - 0.031505407223141116 * t2804 * t118 - 0.09451622166942335 * t2807 - 0.1890324433388467 * t2809 - t2812 + 0.09451622166942335 * t2814 + 0.1890324433388467 * t2816 + t2820;
    (t2814, t2816, t2820, t2821)
}
