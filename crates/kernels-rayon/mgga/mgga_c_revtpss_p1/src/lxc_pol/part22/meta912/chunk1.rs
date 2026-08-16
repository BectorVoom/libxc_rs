//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3118/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3118(t15830: f64, t3111: f64, t11866: f64, t16035: f64, t16088: f64, t342: f64, t380: f64, t16219: f64, t3241: f64, t12047: f64, t53552: f64, t15810: f64, t3127: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55002 = t15830 * t3111;
    let t55004 = t11866 * t16035;
    let t55011 = t342 * t380 * t16088;
    let t55033 = t3241 * t16219;
    let t55046 = t12047 * t53552;
    let t55058 = t3127 * t3172 * t15810;
    (t55002, t55004, t55011, t55033, t55046, t55058)
}
