//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1471/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1471(t1263: f64, t372: f64, t6628: f64, t1260: f64, t20850: f64, t11262: f64, t3600: f64, t6630: f64, t3610: f64, t6634: f64, t5326: f64, t5390: f64) -> (f64, f64, f64, f64, f64) {
    let t69839 = t372 * t1263 * t6628;
    let t69906 = t20850 * t1260;
    let t69910 = t3600 * t11262 * t6630;
    let t69964 = t3610 * t11262 * t6634;
    let t69968 = t5326 * t5390;
    (t69839, t69906, t69910, t69964, t69968)
}
