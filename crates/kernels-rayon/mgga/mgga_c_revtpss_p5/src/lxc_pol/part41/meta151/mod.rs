//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk678;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk679;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta151(t1448: f64, t1450: f64, t565: f64, t2219: f64, t2223: f64, t2226: f64, t2230: f64, t2233: f64, t2239: f64, t1466: f64, t602: f64, t1497: f64, t644: f64, t1469: f64, t606: f64, t30: f64, t33: f64, t70: f64, t2255: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4140, t4146, t4147, t4171, t4173, t4178) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk678(t1448, t1450, t565, t2219, t2223, t2226, t2230, t2233, t2239, t1466, t602, t1497, t644);
        let t4181 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk679(t1469, t606);
        let (t4182, t4186) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk680(t30, t33, t4181, t70, t2255, zeta_threshold);
    (t4140, t4146, t4147, t4171, t4173, t4178, t4181, t4182, t4186)
}
