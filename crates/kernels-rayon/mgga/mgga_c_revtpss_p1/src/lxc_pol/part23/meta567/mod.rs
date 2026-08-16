//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2146;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2147;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2148;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta567(t22852: f64, t4012: f64, t828: f64, t1882: f64, t6861: f64, t9994: f64, t1390: f64, t4003: f64, t1370: f64, t13798: f64, t13801: f64, t1410: f64, t22038: f64, t22044: f64, t22057: f64, t22059: f64, t22063: f64, t22069: f64, t22843: f64, t22849: f64, t4002: f64, t5671: f64, t9735: f64, t9993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22854, t22857) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2146(t22852, t4012, t828, t1882, t6861);
        let t22858 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2147(t22857, t9994);
        let (t22860, t22863) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2148(t1390, t22858, t828, t22857, t4003);
        let (t22865, t22874) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2149(t1390, t22863, t828, t1370, t13798, t13801, t1410, t22038, t22044, t22057, t22059, t22063, t22069, t22843, t22849, t22854, t22860, t4002, t5671, t9735, t9993);
    (t22854, t22857, t22858, t22860, t22863, t22865, t22874)
}
