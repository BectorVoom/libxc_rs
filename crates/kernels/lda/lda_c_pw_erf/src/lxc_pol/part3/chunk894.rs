//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 894/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk894<F: Float>(t325: F, t4625: F, t4611: F, t4606: F, t4621: F, t11: F, t11687: F, t1243: F, t11691: F, t1953: F, t2954: F, t739: F, t9763: F, t9836: F, t34: F, t3518: F, t940: F) -> (F, F, F, F, F, F, F, F) {
    let t11711 = t325 * t4625;
    let t11713 = t325 * t4611;
    let t11715 = t4606 * t4621;
    let t11718 = t11 * t1243 * t11687;
    let t11721 = t1953 * t1243 * t11691;
    let t11724 = t9763 * t739 * t2954;
    let t11726 = t11 * t9836 * t11724;
    let t11729 = t3518 * t34 * t940;
    (t11711, t11713, t11715, t11718, t11721, t11724, t11726, t11729)
}
