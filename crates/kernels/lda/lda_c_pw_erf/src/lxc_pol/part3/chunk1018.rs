//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1018/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1018<F: Float>(t13607: F, t13647: F, t13692: F, t13740: F, t186: F, t211: F, t582: F, t1513: F, t2100: F, t1284: F, t4571: F, t10011: F, t4484: F, t3407: F, t3965: F, t4483: F) -> (F, F, F, F, F) {
    let t13746 = 2.0 / 15.0 * t211 * t186 * t582 * (t13607 + t13647 + t13692 + t13740);
    let t13748 = 4.0 / 5.0 * t1513 * t2100;
    let t13749 = t1284 * t4571;
    let t13750 = 8.0 / 45.0 * t13749;
    let t13751 = t10011 * t4484;
    let t13752 = 32.0 / 45.0 * t13751;
    let t13755 = 16.0 / 15.0 * t3965 * t4483 * t3407;
    (t13746, t13748, t13750, t13752, t13755)
}
