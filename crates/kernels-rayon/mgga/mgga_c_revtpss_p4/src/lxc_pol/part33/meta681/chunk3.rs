//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2224/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2224(t116: f64, t30715: f64, t108078: f64, t108080: f64, t108083: f64, t108085: f64, t108087: f64, t108089: f64, t108099: f64, t108103: f64, t108105: f64, t108107: f64, t108109: f64, t108111: f64, t108117: f64, t1843: f64, t29422: f64, t29456: f64, t30944: f64, t4248: f64, t4292: f64, t649: f64, t651: f64, t671: f64, t7732: f64, t8233: f64) -> (f64, f64) {
    let t111696 = t30715 * t116;
    let t111704 = -4.0_f64 * t4292 * t651 * t8233 - 2.0_f64 * t111696 * t671 - 2.0_f64 * t1843 * t29422 - 4.0_f64 * t29456 * t4248 - 4.0_f64 * t29456 * t7732 - t30944 * t649 - t108078 - t108080 - t108083 - t108085 - t108087 - t108089 - t108099 + t108103 - t108105 - t108107 - t108109 - t108111 - t108117;
    (t111696, t111704)
}
