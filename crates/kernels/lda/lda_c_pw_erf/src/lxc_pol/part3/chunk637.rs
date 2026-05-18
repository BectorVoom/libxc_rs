//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 637/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk637<F: Float>(t3762: F, t576: F, t571: F, t1469: F, t3416: F, t1287: F, t581: F, t593: F, t1466: F, t1318: F, t1278: F, t529: F) -> (F, F, F, F, F, F, F, F) {
    let t3763 = t3762 * t576;
    let t3764 = t571 * t3763;
    let t3765 = F::new(8.0) / F::new(135.0) * t3764;
    let t3767 = F::new(8.0) / F::new(5.0) * t3416 * t1469;
    let t3769 = t581 * t1287 * t593;
    let t3770 = t1466 * t3769;
    let t3772 = F::new(4.0) / F::new(5.0) * t1318 * t3770;
    let t3773 = t529 * t1278;
    (t3763, t3764, t3765, t3767, t3769, t3770, t3772, t3773)
}
