//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1160/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1160<F: Float>(t325: F, t4681: F, t4667: F, t4606: F, t4677: F, t11: F, t12264: F, t1349: F, t12153: F, t1953: F, t2967: F, t743: F, t9410: F) -> (F, F, F, F, F, F) {
    let t13585 = t325 * t4681;
    let t13587 = t325 * t4667;
    let t13589 = t4606 * t4677;
    let t13592 = t11 * t1349 * t12264;
    let t13595 = t1953 * t1349 * t12153;
    let t13598 = t9410 * t743 * t2967;
    (t13585, t13587, t13589, t13592, t13595, t13598)
}
