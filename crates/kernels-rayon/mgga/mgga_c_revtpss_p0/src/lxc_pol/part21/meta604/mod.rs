//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2334;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta604(t10960: f64, t2435: f64, t2482: f64, t39620: f64, t686: f64, t72: f64, t879: f64, t10073: f64, t10934: f64, t253: f64, t39552: f64, t2783: f64, t9646: f64, t22: f64, t251: f64, t837: f64, t2722: f64, t860: f64, t231: f64, t2782: f64, t10665: f64, t2723: f64, t4503: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39687, t39692, t39694, t39697, t39698) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2334(t10960, t2435, t2482, t39620, t686, t72, t879, t10073, t10934, t253, t39552, t2783, t9646);
        let (t39701, t39704, t39707, t39709, t39712) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2335(t22, t251, t39698, t837, t2722, t860, t231, t2782, t2783, t10665, t2723, t4503);
    (t39687, t39692, t39694, t39697, t39698, t39701, t39704, t39707, t39709, t39712)
}
