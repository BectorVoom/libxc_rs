//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 993/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk993<F: Float>(t25240: F, t2693: F, t2710: F, t228: F, t25273: F, t802: F, t25277: F, t2707: F, t10896: F, t7025: F, t25282: F, t9802: F, t243: F, t7021: F, t2732: F, t1941: F, t853: F) -> (F, F, F, F, F, F, F) {
    let t92966 = t2710 * t25240 * t2693;
    let t92968 = t25273 * t228;
    let t92969 = t92968 * t802;
    let t92971 = t25277 * t2707;
    let t92973 = t7025 * t10896;
    let t92975 = t9802 * t25282;
    let t92978 = t7021 * t243;
    let t92979 = t92978 * t2732;
    let t92981 = t1941 * t853;
    (t92966, t92969, t92971, t92973, t92975, t92979, t92981)
}
