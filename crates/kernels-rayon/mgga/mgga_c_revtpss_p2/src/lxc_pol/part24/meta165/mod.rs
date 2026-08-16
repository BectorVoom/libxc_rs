//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta165 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk818;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta165(t300: f64, t6541: f64, t6514: f64, t1765: f64, t5192: f64, t1188: f64, t3495: f64, t6518: f64, t1196: f64, t1179: f64, t6534: f64, t3520: f64, t3523: f64, t3546: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64, t459: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk818(t300, t6541, t6514, t1765, t5192, t1188, t3495, t6518, t1196, t1179, t6534, t3520);
        let (t6556, t6558, t6563, t6564) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk819(t3523, t6555, t1196, t3546, t5044, t6423, t6427, t6431, t459);
    (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555, t6556, t6558, t6563, t6564)
}
