//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk562;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk563;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk564;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta107(t2629: f64, t2630: f64, t73: f64, t853: f64, t820: f64, t843: f64, t849: f64, t857: f64, t212: f64, t27: f64, t225: f64, t816: f64, t240: f64, t823: f64, t243: f64, t836: f64, t231: f64, t596: f64, t813: f64, t2482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2632, t2638, t2652) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk562(t2629, t2630, t73, t853, t820, t843, t849);
        let (t2653, t2661) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk563(t2652, t857, t212, t27, t225, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk564(t240, t823);
        let (t2664, t2665, t2666, t2668, t2672, t2674) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk565(t243, t836, t231, t2662, t2661, t240, t596, t816, t813, t2482, t27, t849);
    (t2632, t2638, t2652, t2653, t2661, t2662, t2664, t2665, t2666, t2668, t2672, t2674)
}
