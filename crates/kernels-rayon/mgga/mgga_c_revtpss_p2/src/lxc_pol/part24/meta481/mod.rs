//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1470;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta481(t17376: f64, t17524: f64, t17528: f64, t3140: f64, t6564: f64, t3599: f64, t17361: f64, t5274: f64, t1234: f64, t21271: f64, t21093: f64, t372: f64, t1263: f64, t6628: f64, t1260: f64, t20850: f64, t11262: f64, t3600: f64, t6630: f64, t3610: f64, t6634: f64, t5326: f64, t5390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69680, t69683, t69692, t69693, t69700, t69795, t69832) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1470(t17376, t17524, t17528, t3140, t6564, t3599, t17361, t5274, t1234, t21271, t21093, t372);
        let (t69839, t69906, t69910, t69964, t69968) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1471(t1263, t372, t6628, t1260, t20850, t11262, t3600, t6630, t3610, t6634, t5326, t5390);
    (t69680, t69683, t69692, t69693, t69700, t69795, t69832, t69839, t69906, t69910, t69964, t69968)
}
