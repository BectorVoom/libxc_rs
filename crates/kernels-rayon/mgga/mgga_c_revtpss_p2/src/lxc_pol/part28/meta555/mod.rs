//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2007;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta555(t25338: f64, t689: f64, t887: f64, t2439: f64, t25334: f64, t7036: f64, t820: f64, t844: f64, t2751: f64, t2482: f64, t814: f64, t10782: f64, t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64, t228: f64, t25273: f64, t802: f64, t25277: f64, t2707: f64, t25282: f64, t9802: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92930, t92935, t92951, t92952, t92955, t92956) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2007(t25338, t689, t887, t2439, t25334, t7036, t820, t844, t2751, t2482, t814, t10782);
        let (t92963, t92966, t92968, t92969, t92971, t92975) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2008(t10744, t2664, t7028, t25240, t2693, t2710, t228, t25273, t802, t25277, t2707, t25282, t9802);
    (t92930, t92935, t92951, t92952, t92955, t92956, t92963, t92966, t92968, t92969, t92971, t92975)
}
