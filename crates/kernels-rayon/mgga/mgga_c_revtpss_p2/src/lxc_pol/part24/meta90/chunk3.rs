//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 530/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk530(t136: f64, t251: f64, t2457: f64, t2710: f64, t2783: f64, t786: f64) -> (f64, f64, f64, f64) {
    let t2793 = t251 * t136;
    let t2796 = 0.11565819519348392139e-2_f64 * t2710 * t2793 * t2457;
    let t2797 = t2783 * t251;
    let t2798 = t786 * t2797;
    (t2793, t2796, t2797, t2798)
}
