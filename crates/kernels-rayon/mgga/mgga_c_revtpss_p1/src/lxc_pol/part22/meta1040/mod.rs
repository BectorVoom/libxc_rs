//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1040 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3632;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1040(t12227: f64, t3385: f64, t6474: f64, t16942: f64, t1733: f64, t3384: f64, t12248: f64, t3427: f64, t20651: f64, t44017: f64, t6471: f64, t20644: f64, t3433: f64, t68738: f64, t68742: f64, t68744: f64, t68746: f64, t68748: f64, t68751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68754, t68757, t68760, t68763, t68766, t68769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3632(t12227, t3385, t6474, t16942, t1733, t3384, t12248, t3427, t20651, t44017, t6471, t20644);
        let (t68772, t68773) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3633(t20644, t3427, t3433, t68738, t68742, t68744, t68746, t68748, t68751, t68754, t68757, t68760, t68763, t68766, t68769);
    (t68754, t68757, t68760, t68763, t68766, t68769, t68772, t68773)
}
