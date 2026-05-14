//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1352/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1352<F: Float>(t19583: F, t5592: F, t19563: F, t19564: F, t19565: F, t19566: F, t19567: F, t19572: F, t19575: F, t19578: F, t19580: F, t8999: F, t9019: F, t9022: F, t156: F, t426: F, t7129: F) -> (F, F) {
    let t19584 = t5592 * t19583;
    let t19586 = 3.91744 * t8999 + t19563 - t19564 + t19565 - t19566 - t19567 + 4.570346666666667 * t9019 + 2.0 / 3.0 * t9022 - t19572 + t19575 + t19578 + 29.3808 * t19580 - 11.75232 * t19584;
    let t19590 = t426 * t156 * t7129;
    (t19586, t19590)
}
