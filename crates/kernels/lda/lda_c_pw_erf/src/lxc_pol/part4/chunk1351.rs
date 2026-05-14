//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1351/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1351<F: Float>(t9003: F, t9005: F, t9011: F, t9015: F, t9017: F, t1652: F, t2599: F, t933: F, t2611: F, t325: F, t415: F, t7126: F, t431: F, t5594: F, t7116: F, t1: F, t1832: F, t322: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19563 = 3.031285185185185 * t9003;
    let t19564 = 0.6495611111111111 * t9005;
    let t19565 = 1.2991222222222223 * t9011;
    let t19566 = 0.6495611111111111 * t9015;
    let t19567 = 2.5982444444444446 * t9017;
    let t19571 = t1652 * t2599 * t933;
    let t19572 = 0.6495611111111111 * t19571;
    let t19574 = t1652 * t2611 * t933;
    let t19575 = 0.3247805555555556 * t19574;
    let t19577 = t415 * t7126 * t325;
    let t19578 = 0.9743416666666667 * t19577;
    let t19580 = t431 * t7116 * t5594;
    let t19583 = t1832 * t1 * t322;
    (t19563, t19564, t19565, t19566, t19567, t19572, t19575, t19578, t19580, t19583)
}
