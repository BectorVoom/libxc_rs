//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1389/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1389(t21332: f64, t459: f64, t225: f64, t480: f64, t12832: f64, t17401: f64, t17736: f64, t17767: f64, t17771: f64, t17791: f64, t17792: f64, t21300: f64, t21306: f64, t21310: f64, t21313: f64, t21316: f64, t3718: f64, t484: f64, t5335: f64, t5348: f64, t6690: f64) -> (f64, f64) {
    let t21333 = t21332 * t459;
    let t21334 = t21333 * t225;
    let t21335 = t21334 * t480;
    let t21338 = -0.42874018118069736972e-3_f64 * t17401 * t5348 - 0.21437009059034868486e-3_f64 * t3718 * t21300 - 0.42874018118069736972e-3_f64 * t12832 * t6690 - t17767 - t17771 - t17791 + t17792 / 81.0_f64 - 0.42874018118069736972e-3_f64 * t21306 * t5335 - 0.57165357490759649296e-3_f64 * t17736 * t21310 + 0.72409452821628889107e-2_f64 * t21313 * t484 - 0.22866142996303859718e-2_f64 * t21316 * t484 + 0.21437009059034868486e-3_f64 * t21335 * t484;
    (t21333, t21338)
}
