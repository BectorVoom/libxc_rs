//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 990/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk990<F: Float>(t1318: F, t3899: F, t5366: F, t2089: F, t933: F, t1973: F, t925: F, t325: F, t4625: F, t4611: F, t4606: F, t4621: F, t4638: F, t1968: F, t4616: F, t5021: F, t5103: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11680 = t1318 * t3899 * t5366;
    let t11695 = t933 * t2089;
    let t11709 = t925 * t1973;
    let t11711 = t325 * t4625;
    let t11713 = t325 * t4611;
    let t11715 = t4606 * t4621;
    let t11751 = t325 * t4638;
    let t11753 = t925 * t1968;
    let t11755 = t325 * t4616;
    let t11757 = t5021 * t5103;
    (t11680, t11695, t11709, t11711, t11713, t11715, t11751, t11753, t11755, t11757)
}
