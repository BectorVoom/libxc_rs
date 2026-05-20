//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2037/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2037<F: Float>(t11970: F, t1973: F, t1058: F, t25554: F, t3201: F, t7126: F, t25561: F, t7114: F, t25566: F, t1024: F, t25576: F, t25525: F, t3123: F) -> (F, F, F, F, F, F, F, F) {
    let t93611 = F::cast_from(0.1270341277572436651e-3_f64) * t1973 * t11970;
    let t93616 = t25554 * t1058;
    let t93618 = t7126 * t3201;
    let t93620 = t25561 * t1058;
    let t93622 = t7114 * t3201;
    let t93627 = t25566 * t1058;
    let t93646 = t1024 * t25576;
    let t93649 = t3123 * t25525;
    (t93611, t93616, t93618, t93620, t93622, t93627, t93646, t93649)
}
