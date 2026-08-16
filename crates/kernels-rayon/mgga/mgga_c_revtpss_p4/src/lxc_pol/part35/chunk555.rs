//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 555/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk555(t2672: f64, t2686: f64, t2691: f64, t2730: f64, t4359: f64, t4373: f64, t4455: f64, t5980: f64, t5985: f64, t5989: f64, t5993: f64, t6040: f64, t799: f64, t825: f64, t851: f64) -> f64 {
    let t6041 = -0.21437009059034868486e-3_f64 * t825 * t5980 + 0.20007875121765877254e-2_f64 * t4359 - t799 * t5985 / 48.0_f64 + t2730 * t5989 / 16.0_f64 + 0.42874018118069736972e-2_f64 * t851 * t5993 - t2672 + t2686 + 0.57165357490759649296e-4_f64 * t4373 + t2691 + 7.0_f64 / 72.0_f64 * t4455 + t6040;
    t6041
}
