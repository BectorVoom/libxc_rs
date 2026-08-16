//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1118/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1118(t10807: f64, t25270: f64, t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64, t228: f64, t25273: f64, t802: f64, t25277: f64, t2707: f64) -> (f64, f64, f64, f64, f64) {
    let t92960 = t25270 * t10807;
    let t92963 = t10744 * t7028 * t2664;
    let t92966 = t2710 * t25240 * t2693;
    let t92968 = t25273 * t228;
    let t92969 = t92968 * t802;
    let t92971 = t25277 * t2707;
    (t92960, t92963, t92966, t92969, t92971)
}
