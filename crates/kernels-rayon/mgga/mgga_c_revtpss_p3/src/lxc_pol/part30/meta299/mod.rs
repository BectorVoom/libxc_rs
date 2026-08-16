//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta299 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta299(t550: f64, t9721: f64, t268: f64, t9718: f64, t64: f64, t8779: f64, t159: f64, t535: f64, t2236: f64, t65: f64, t235: f64, t1389: f64, t3964: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9723, t9725, t9727, t9729, t9731, t9732, t9735) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1282(t550, t9721, t268, t9718, t64, t8779, t159, t535, t2236, t65, t235, t1389, t3964);
    (t9723, t9725, t9727, t9729, t9731, t9732, t9735)
}
