//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1489;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta275(t760: f64, t9372: f64, t37: f64, t716: f64, t2523: f64, t2626: f64, t9425: f64, t2609: f64, t606: f64, t706: f64, t2475: f64, t73: f64, t775: f64, t853: f64, t2710: f64, t2793: f64, t9285: f64, t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64, t2718: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10604, t10605, t10608, t10611, t10612, t10613, t10626) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1489(t760, t9372, t37, t716, t2523, t2626, t9425, t2609, t606, t706, t2475, t73);
        let (t10631, t10645, t10647, t10651, t10661) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1490(t775, t853, t2710, t2793, t9285, t2470, t2804, t874, t875, t9288, t2718, t860);
    (t10604, t10605, t10608, t10611, t10612, t10613, t10626, t10631, t10645, t10647, t10651, t10661)
}
