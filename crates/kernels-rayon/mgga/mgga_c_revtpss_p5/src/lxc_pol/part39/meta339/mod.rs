//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1136;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta339(t2371: f64, t93: f64, t1514: f64, t2289: f64, t4264: f64, t625: f64, t4288: f64, t10208: f64, t1513: f64, t2340: f64, t2339: f64, t4287: f64, t665: f64, t2366: f64, t4263: f64, t10227: f64, t1504: f64, t2350: f64, t2349: f64, t97: f64, t2255: f64, t658: f64, t2256: f64, t4269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13440, t13448, t13451, t13453, t13455, t13458) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1136(t2371, t93, t1514, t2289, t4264, t625, t4288, t10208, t1513, t2340, t2339, t4287);
        let (t13459, t13462, t13472, t13475, t13476, t13479) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1137(t13458, t665, t2366, t4263, t10227, t1504, t2350, t2349, t97, t2255, t658, t2256, t4269);
    (t13440, t13448, t13451, t13453, t13455, t13459, t13462, t13472, t13475, t13476, t13479)
}
