//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1011/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1011<F: Float>(t11: F, t12254: F, t3633: F, t13344: F, t1349: F, t1953: F, t13294: F, t557: F, t190: F, t4981: F, t9821: F, t325: F, t4681: F, t4667: F, t4606: F, t4677: F) -> (F, F, F, F, F, F, F) {
    let t13574 = t11 * t3633 * t12254;
    let t13577 = t1953 * t1349 * t13344;
    let t13580 = t1953 * t557 * t13294;
    let t13583 = t190 * t9821 * t4981;
    let t13585 = t325 * t4681;
    let t13587 = t325 * t4667;
    let t13589 = t4606 * t4677;
    (t13574, t13577, t13580, t13583, t13585, t13587, t13589)
}
