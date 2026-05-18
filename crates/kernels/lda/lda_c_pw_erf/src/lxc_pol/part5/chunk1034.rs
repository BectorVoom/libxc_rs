//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1034/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1034<F: Float>(t504: F, t6590: F, t108: F, t181: F, t518: F, t6630: F, t1325: F, t3787: F, t6916: F, t6980: F, t2478: F, t3975: F) -> (F, F, F, F, F, F) {
    let t18133 = t6590 * t504;
    let t18138 = t181 * t108;
    let t18154 = t6630 * t518;
    let t18158 = t1325 * t3787 * t6916;
    let t18163 = t1325 * t3787 * t6980;
    let t18184 = t3975 * t2478;
    (t18133, t18138, t18154, t18158, t18163, t18184)
}
