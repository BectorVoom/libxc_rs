//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2555/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2555(t1062: f64, t43154: f64, t16088: f64, t342: f64, t380: f64, t16219: f64, t3241: f64, t1063: f64, t11262: f64, t4802: f64, t4807: f64, t11773: f64, t15925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54982 = t43154 * t1062;
    let t55011 = t342 * t380 * t16088;
    let t55033 = t3241 * t16219;
    let t55034 = t55033 / 162.0_f64;
    let t55061 = t1063 * t11262 * t4802;
    let t55062 = 0.19055119163586549765e-3_f64 * t55061;
    let t55064 = t1063 * t11262 * t4807;
    let t55065 = 0.15879265969655458138e-3_f64 * t55064;
    let t55141 = t15925 * t11773;
    (t54982, t55011, t55034, t55062, t55065, t55141)
}
