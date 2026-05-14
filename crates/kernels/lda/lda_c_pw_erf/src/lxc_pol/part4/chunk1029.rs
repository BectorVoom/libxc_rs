//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1029/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1029<F: Float>(t1953: F, t817: F, t1955: F, t8930: F, t1284: F, t4571: F, t10011: F, t4484: F, t493: F, t9248: F, t3704: F, t4505: F) -> (F, F, F, F, F, F) {
    let t13731 = t1953 * t817;
    let t13736 = t8930 * t1955;
    let t13749 = t1284 * t4571;
    let t13751 = t10011 * t4484;
    let t13767 = t493 * t9248;
    let t13771 = t4505 * t3704;
    (t13731, t13736, t13749, t13751, t13767, t13771)
}
