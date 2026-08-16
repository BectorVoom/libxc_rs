//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3689/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3689(t17544: f64, t5293: f64, t17373: f64, t21275: f64, t17769: f64, t5381: f64, t5391: f64, t1247: f64, t20902: f64, t3172: f64, t1234: f64, t21271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69773 = t5293 * t17544;
    let t69783 = t21275 * t17373;
    let t69787 = t5381 * t17769;
    let t69789 = t5391 * t17769;
    let t69793 = t1247 * t3172 * t20902;
    let t69795 = t1234 * t21271;
    (t69773, t69783, t69787, t69789, t69793, t69795)
}
