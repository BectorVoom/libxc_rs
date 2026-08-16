//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2081/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2081(t13429: f64, t13521: f64, t13532: f64, t13540: f64, t1519: f64, t2007: f64, t2320: f64, t2328: f64, t2331: f64, t25805: f64, t27830: f64, t28030: f64, t4297: f64, t508: f64, t649: f64, t671: f64, t6985: f64, t7883: f64, t92737: f64, t97593: f64, t97604: f64, t97606: f64, t97608: f64, t97610: f64, t97617: f64, t97622: f64, t97629: f64, t97632: f64) -> f64 {
    let t97635 = -2.0_f64 * t13429 * t2007 - 2.0_f64 * t13521 * t6985 - 4.0_f64 * t13532 * t6985 - 4.0_f64 * t13540 * t6985 - 2.0_f64 * t1519 * t92737 - 4.0_f64 * t1519 * t97632 - t2320 * t7883 - 2.0_f64 * t2328 * t7883 - 4.0_f64 * t2331 * t28030 - 4.0_f64 * t25805 * t4297 - 2.0_f64 * t27830 * t649 - 2.0_f64 * t508 * t97593 - 4.0_f64 * t671 * t97622 - t97604 - t97606 - t97608 - t97610 - t97617 - t97629;
    t97635
}
