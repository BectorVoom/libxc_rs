//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1309;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1310;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta360(t10073: f64, t10934: f64, t253: f64, t39552: f64, t2783: f64, t9646: f64, t22: f64, t251: f64, t837: f64, t2722: f64, t860: f64, t231: f64, t2782: f64, t10665: f64, t2723: f64, t4503: f64, t10638: f64, t10111: f64, t2789: f64, t588: f64, t870: f64, t10963: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39694, t39697, t39701, t39704, t39707) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1309(t10073, t10934, t253, t39552, t2783, t9646, t22, t251, t837, t2722, t860, t231, t2782);
        let (t39709, t39712, t39714, t39719, t39723, t39724) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1310(t10665, t251, t2723, t2782, t4503, t10638, t10111, t22, t2789, t588, t870, t10963, t9303);
    (t39694, t39697, t39701, t39704, t39707, t39709, t39712, t39714, t39719, t39723, t39724)
}
