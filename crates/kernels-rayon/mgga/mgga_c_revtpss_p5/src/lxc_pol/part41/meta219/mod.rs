//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk849;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk850;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk851;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk852;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk853;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta219(t5819: f64, t70: f64, t17: f64, t2255: f64, t30: f64, t33: f64, zeta_threshold: f64, t36: f64, t1470: f64, t1486: f64, t2275: f64, t48: f64, t476: f64, t53: f64, t2282: f64, t60: f64, sigma2: f64, t1480: f64, t1483: f64, t2290: f64, t44: f64, t56: f64, t61: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5820, t5823) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk849(t5819, t70, t17, t2255);
        let t5824 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk850(t5823);
        let t5825 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk851(t30, t33, t5824, zeta_threshold);
        let (t5826, t5827, t5830, t5835, t5838, t5843, t5848, t5851) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk852(t36, t5825, t70, t1470, t1486, t2275, t5819, t48, t476, t53, t2282, t60, sigma2);
        let (t5854, t5855) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk853(t1480, t1483, t2290, t44, t56, t5835, t5838, t5843, t5848, t5851, t61, t38);
    (t5820, t5823, t5824, t5825, t5826, t5827, t5830, t5835, t5838, t5843, t5854, t5855)
}
