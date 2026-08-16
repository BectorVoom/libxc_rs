//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2439;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2440;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta581(t18534: f64, t18553: f64, t18568: f64, t18583: f64, t225: f64, t1553: f64, t73: f64, t2475: f64, t5966: f64, t775: f64, t4343: f64, t4416: f64, t5962: f64, t853: f64, t18392: f64, t832: f64, t1555: f64, t227: f64, t229: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t830: f64, t833: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18586, t18592, t18599, t18600, t18603) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2439(t18534, t18553, t18568, t18583, t225, t1553, t73, t2475, t5966, t775, t4343, t4416);
        let (t18608, t18609, t18612, t18615) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2440(t5962, t853, t775, t18392, t832, t1553, t1555, t18586, t18592, t18600, t18603, t227, t229, t4409, t4415, t4417, t4420, t6006, t6010, t6013, t830, t833);
        let t18616 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2441(t18615, t231);
    (t18586, t18592, t18599, t18600, t18603, t18608, t18609, t18612, t18615, t18616)
}
