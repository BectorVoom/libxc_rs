//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1265/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1265<F: Float>(t3291: F, t6258: F, t1082: F, t19380: F, t6271: F, t73: F, t4976: F, t11249: F, t6305: F) -> (F, F, F, F) {
    let t19438 = t3291 * t6258;
    let t19443 = t1082 * t19380;
    let t19446 = t6271 * t73;
    let t19447 = t19446 * t4976;
    let t19450 = t6305 * t11249;
    (t19438, t19443, t19447, t19450)
}
