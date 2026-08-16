//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2143;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2144;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta566<F: Float>(t22813: F, t828: F, t9942: F, t1414: F, t22809: F, t22079: F, t3936: F, t6869: F, t13790: F, t5673: F, t1883: F, t22074: F, t13765: F, t13779: F, t13781: F, t1410: F, t22023: F, t22028: F, t22030: F, t3934: F, t5671: F, t9711: F, t9725: F, t9729: F, t1868: F, t4003: F, t22046: F, t124: F, t800: F, t6816: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22815, t22822, t22829, t22833, t22837) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2143::<F>(t22813, t828, t9942, t1414, t22809, t22079, t3936, t6869, t13790, t5673, t1883, t22074);
        let t22840 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2144::<F>(t13765, t13779, t13781, t1410, t22023, t22028, t22030, t22815, t22822, t22829, t22833, t22837, t3934, t5671, t9711, t9725, t9729);
        let (t22841, t22843, t22848, t22849, t22852) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2145::<F>(t1868, t4003, t22046, t3936, t124, t22809, t800, t6816);
    (t22815, t22822, t22829, t22833, t22837, t22840, t22841, t22843, t22848, t22849, t22852)
}
