//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 686/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk686(t2662: f64, t2664: f64, t2661: f64, t240: f64, t596: f64, t243: f64, t816: f64, t813: f64, t2482: f64, t27: f64, t849: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2665 = t2662 * t2664;
    let t2666 = t2661 * t2665;
    let t2668 = t596 * t240;
    let t2670 = t2668 * t243 * t816;
    let t2672 = 0.13552000749142754193e-3_f64 * t813 * t2670;
    let t2674 = t2482 * t849 * t27;
    (t2665, t2666, t2668, t2670, t2672, t2674)
}
