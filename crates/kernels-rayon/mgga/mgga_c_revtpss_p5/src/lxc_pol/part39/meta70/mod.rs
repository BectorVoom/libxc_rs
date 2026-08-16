//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta70 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk422;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk423;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk424;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk425;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta70(t212: f64, t555: f64, t225: f64, t561: f64, t689: f64, t556: f64, t786: f64, t72: f64, t686: f64, t535: f64, t795: f64, t159: f64, t540: f64, t216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1357 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk422(t212, t555);
        let t1358 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk423(t225, t561);
        let (t1359, t1361, t1362, t1363, t1364) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk424(t1357, t1358, t689, t556, t786, t561, t72, t686);
        let (t1366, t1368, t1369, t1370) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk425(t1362, t1364, t535, t795, t159, t540, t216);
    (t1357, t1358, t1359, t1361, t1362, t1363, t1364, t1366, t1368, t1369, t1370)
}
