//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1005/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1005<F: Float>(t325: F, t4638: F, t1968: F, t925: F, t4616: F, t5021: F, t5103: F, t331: F, t5106: F, t5109: F, t4619: F, t940: F) -> (F, F, F, F, F, F, F, F) {
    let t11751 = t325 * t4638;
    let t11753 = t925 * t1968;
    let t11754 = F::cast_from(0.03199259259259259_f64) * t11753;
    let t11755 = t325 * t4616;
    let t11757 = t5021 * t5103;
    let t11762 = t331 * t5106;
    let t11764 = t5021 * t5109;
    let t11766 = t4619 * t940;
    (t11751, t11753, t11754, t11755, t11757, t11762, t11764, t11766)
}
