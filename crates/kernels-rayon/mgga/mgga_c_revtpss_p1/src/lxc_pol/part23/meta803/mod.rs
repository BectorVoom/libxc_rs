//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta803 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2632;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta803(t2782: f64, t2797: f64, t62637: f64, t18615: f64, t251: f64, t231: f64, t2783: f64, t10069: f64, t18738: f64, t18742: f64, t10073: f64, t10530: f64, t18718: f64, t2470: f64, t18719: f64, t39609: f64, t18761: f64, t874: f64, t14602: f64, t2482: f64, t2811: f64, t5977: f64, t2801: f64, t879: f64, t18750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62639, t62641, t62644, t62649, t62651, t62653, t62665) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2632(t2782, t2797, t62637, t18615, t251, t231, t2783, t10069, t18738, t18742, t10073, t10530, t18718, t2470);
        let (t62667, t62670, t62675, t62682, t62684) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2633(t18719, t39609, t18761, t2470, t874, t14602, t2482, t2811, t5977, t2801, t879, t10073, t18750);
    (t62639, t62641, t62644, t62649, t62651, t62653, t62665, t62667, t62670, t62675, t62682, t62684)
}
