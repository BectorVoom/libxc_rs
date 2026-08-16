//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2713/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2713(t20879: f64, t3172: f64, t3711: f64, t1260: f64, t20850: f64, t11262: f64, t3600: f64, t6630: f64, t17225: f64, t5391: f64, t21183: f64, t20875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69899 = t3711 * t3172 * t20879;
    let t69906 = t20850 * t1260;
    let t69910 = t3600 * t11262 * t6630;
    let t69916 = t5391 * t17225;
    let t69936 = t3711 * t3172 * t21183;
    let t69939 = t3711 * t3172 * t20875;
    (t69899, t69906, t69910, t69916, t69936, t69939)
}
