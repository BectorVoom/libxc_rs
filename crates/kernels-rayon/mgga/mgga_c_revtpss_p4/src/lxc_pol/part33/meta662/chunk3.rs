//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2159/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2159(t2322: f64, t30005: f64, t4254: f64, t1310: f64, t30004: f64, t651: f64, t27123: f64, t7742: f64, t27126: f64, t28063: f64, t7732: f64, t28056: f64, t4248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108078 = 2.0_f64 * t2322 * t30005;
    let t108080 = 2.0_f64 * t4254 * t30005;
    let t108083 = 2.0_f64 * t651 * t1310 * t30004;
    let t108085 = 4.0_f64 * t27123 * t7742;
    let t108087 = 4.0_f64 * t27126 * t7742;
    let t108089 = 4.0_f64 * t7732 * t28063;
    let t108099 = 4.0_f64 * t4248 * t28056;
    (t108078, t108080, t108083, t108085, t108087, t108089, t108099)
}
