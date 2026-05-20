//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2146;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2147;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2148;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta567<F: Float>(t22852: F, t4012: F, t828: F, t1882: F, t6861: F, t9994: F, t1390: F, t4003: F, t1370: F, t13798: F, t13801: F, t1410: F, t22038: F, t22044: F, t22057: F, t22059: F, t22063: F, t22069: F, t22843: F, t22849: F, t4002: F, t5671: F, t9735: F, t9993: F) -> (F, F, F, F, F, F, F) {
        let (t22854, t22857) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2146::<F>(t22852, t4012, t828, t1882, t6861);
        let t22858 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2147::<F>(t22857, t9994);
        let (t22860, t22863) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2148::<F>(t1390, t22858, t828, t22857, t4003);
        let (t22865, t22874) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2149::<F>(t1390, t22863, t828, t1370, t13798, t13801, t1410, t22038, t22044, t22057, t22059, t22063, t22069, t22843, t22849, t22854, t22860, t4002, t5671, t9735, t9993);
    (t22854, t22857, t22858, t22860, t22863, t22865, t22874)
}
