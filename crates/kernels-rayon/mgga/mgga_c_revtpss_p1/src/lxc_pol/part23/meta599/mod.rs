//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2247;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta599(t1045: f64, t23997: f64, t3117: f64, t1651: f64, t6305: f64, t3155: f64, t3162: f64, t11765: f64, t22688: f64, t1012: f64, t23598: f64, t373: f64, t371: f64, t372: f64, t6244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23998, t23999, t24007) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2247(t1045, t23997, t3117, t1651, t6305);
        let (t24008, t24009, t24012, t24013, t24016, t24017, t24022, t24024, t24031) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2248(t24007, t3155, t3117, t3162, t11765, t22688, t1012, t23598, t373, t371, t372, t1651, t6244);
    (t23998, t23999, t24007, t24008, t24009, t24012, t24013, t24016, t24017, t24022, t24024, t24031)
}
