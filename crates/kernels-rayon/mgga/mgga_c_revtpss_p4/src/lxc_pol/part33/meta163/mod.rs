//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk818;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta163(t2723: f64, t836: f64, t4365: f64, t4364: f64, t1544: f64, t854: f64, t236: f64, t807: f64, t2498: f64, t2518: f64, t2522: f64, t2526: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t4300: f64, t4301: f64, t4304: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t4366 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk818(t2723, t836);
        let (t4368, t4371, t4372, t4373, t4376) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk819(t4365, t4366, t4364, t1544, t854, t236, t807, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304);
    (t4366, t4368, t4371, t4372, t4373, t4376)
}
