//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk872;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk873;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta162(t198: f64, t532: f64, t539: f64, t73: f64, t241: f64, t4000: f64, t820: f64, t550: f64, t72: f64, t245: f64, t1398: f64, t4003: f64, t225: f64, t3999: f64, t213: f64, t4086: f64, t640: f64, t76: f64, t159: f64, t793: f64, t1448: f64, t4147: f64, t587: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5541, t5650, t5671, t5673) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk872(t198, t532, t539, t73, t241, t4000, t820, t550, t72, t245);
        let (t5675, t5744) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk873(t1398, t4003, t225, t3999);
        let (t5745, t5755, t6977, t7021, t7315, t8779) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk874(t213, t5744, t4086, t640, t76, t159, t793, t1448, t4147, t587, t65);
    (t5541, t5650, t5671, t5673, t5675, t5744, t5745, t5755, t6977, t7021, t7315, t8779)
}
