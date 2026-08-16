//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3011/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3011(t12047: f64, t53552: f64, t15810: f64, t3127: f64, t3172: f64, t1063: f64, t11262: f64, t4802: f64, t4807: f64, t11859: f64, t11922: f64, t15894: f64) -> (f64, f64, f64, f64, f64) {
    let t55046 = t12047 * t53552;
    let t55058 = t3127 * t3172 * t15810;
    let t55061 = t1063 * t11262 * t4802;
    let t55062 = 0.19055119163586549765e-3_f64 * t55061;
    let t55064 = t1063 * t11262 * t4807;
    let t55065 = 0.15879265969655458138e-3_f64 * t55064;
    let t55067 = t11859 * t11922 * t15894;
    (t55046, t55058, t55062, t55065, t55067)
}
