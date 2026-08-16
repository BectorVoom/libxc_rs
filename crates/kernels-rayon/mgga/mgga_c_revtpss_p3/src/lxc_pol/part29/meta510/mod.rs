//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1829;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta510(t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64, t228: f64, t25273: f64, t802: f64, t25277: f64, t2707: f64, t25282: f64, t9802: f64, t243: f64, t7021: f64, t2732: f64, t64: f64, t9731: f64, t826: f64, t10631: f64, t10886: f64, t159: f64, t8779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92963, t92966, t92968, t92969, t92971, t92975) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1829(t10744, t2664, t7028, t25240, t2693, t2710, t228, t25273, t802, t25277, t2707, t25282, t9802);
        let (t92979, t92986, t92988, t92991, t92993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1830(t243, t7021, t2732, t64, t9731, t2710, t826, t10631, t10886, t7028, t159, t8779);
    (t92963, t92966, t92968, t92969, t92971, t92975, t92979, t92986, t92988, t92991, t92993)
}
