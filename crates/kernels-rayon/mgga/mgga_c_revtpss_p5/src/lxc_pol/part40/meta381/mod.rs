//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1369;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1370;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1371;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1372;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1373;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta381(t5053: f64, t689: f64, t5057: f64, t12256: f64, t1469: f64, t2251: f64, t12305: f64, t128: f64, t12268: f64, t3360: f64, t3362: f64, t4186: f64, t606: f64, t2258: f64, t5046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t16710 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1369(t5053, t689);
        let (t16711, t16712) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1370(t16710, t5057, t689);
        let (t16713, t16715, t16717) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1371(t16712, t12256, t1469, t2251, t12305, t128);
        let (t16720, t16722) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1372(t12268, t1469, t2251, t3360, t128);
        let (t16725, t16727) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1373(t3362, t4186, t606, t3360, t128);
        let (t16729, t16731) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1374(t2258, t5046, t3360, t128);
    (t16710, t16711, t16712, t16713, t16715, t16717, t16720, t16722, t16725, t16727, t16729, t16731)
}
