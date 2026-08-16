//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk545;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta99(t658: f64, t2349: f64, t100: f64, t2256: f64, t107: f64, t661: f64, t108: f64, t101: f64, t105: f64, t2344: f64, t656: f64, t659: f64, t97: f64, t114: f64, t655: f64, t2335: f64, t2336: f64, t2341: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2350, t2357, t2358, t2359, t2362, t2363, t2366) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk545(t658, t2349, t100, t2256, t107, t661, t108, t101, t105, t2344, t656, t659, t97);
        let (t2367, t2371) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk546(t114, t2366, t655, t2335, t2336, t2341, t69);
    (t2350, t2357, t2358, t2359, t2362, t2363, t2366, t2367, t2371)
}
