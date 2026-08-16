//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta881 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3053;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3054;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta881(t22: f64, t231: f64, t39698: f64, t4494: f64, t2782: f64, t2783: f64, t51375: f64, t10073: f64, t14509: f64, t10069: f64, t40921: f64, t4496: f64, t14537: f64, t10532: f64, t14598: f64, t50511: f64, t2797: f64, t1568: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51676, t51680, t51682, t51684, t51686) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3053(t22, t231, t39698, t4494, t2782, t2783, t51375, t10073, t14509, t10069, t40921, t4496);
        let (t51688, t51696, t51700, t51703, t51708) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3054(t10073, t14537, t10532, t14598, t231, t50511, t2782, t2797, t10069, t1568, t2645, t2783);
    (t51676, t51680, t51682, t51684, t51686, t51688, t51696, t51700, t51703, t51708)
}
