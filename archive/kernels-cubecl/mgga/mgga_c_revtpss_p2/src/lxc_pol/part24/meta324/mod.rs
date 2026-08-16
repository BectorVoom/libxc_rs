//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1125;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1126;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1127;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1128;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta324<F: Float>(t13765: F, t13779: F, t13781: F, t1410: F, t22023: F, t22028: F, t22030: F, t22815: F, t22822: F, t22829: F, t22833: F, t22837: F, t3934: F, t5671: F, t9711: F, t9725: F, t9729: F, t1868: F, t4003: F, t22046: F, t3936: F, t124: F, t22809: F, t800: F, t6816: F, t4012: F, t828: F, t1882: F, t6861: F, t9994: F, t1390: F, t1370: F, t13798: F, t13801: F, t22038: F, t22044: F, t22057: F, t22059: F, t22063: F, t22069: F, t4002: F, t9735: F, t9993: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t22840 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1125::<F>(t13765, t13779, t13781, t1410, t22023, t22028, t22030, t22815, t22822, t22829, t22833, t22837, t3934, t5671, t9711, t9725, t9729);
        let (t22841, t22843, t22849, t22852, t22854, t22857) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1126::<F>(t1868, t4003, t22046, t3936, t124, t22809, t800, t6816, t4012, t828, t1882, t6861);
        let t22858 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1127::<F>(t22857, t9994);
        let (t22860, t22863) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1128::<F>(t1390, t22858, t828, t22857, t4003);
        let (t22865, t22874) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1129::<F>(t1390, t22863, t828, t1370, t13798, t13801, t1410, t22038, t22044, t22057, t22059, t22063, t22069, t22843, t22849, t22854, t22860, t4002, t5671, t9735, t9993);
    (t22840, t22841, t22843, t22849, t22852, t22854, t22857, t22858, t22860, t22863, t22865, t22874)
}
