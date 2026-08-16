//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1227/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1227(t32003: f64, t36479: f64, t38086: f64, t2385: f64, t394: f64, t2132: f64, t7885: f64, t864: f64, t2146: f64, t2147: f64, t31965: f64, t33034: f64, t33037: f64, t33047: f64, t33053: f64, t38073: f64, t38077: f64, t38085: f64, t463: f64, t7912: f64, t7931: f64, t7934: f64, t8069: f64, t8119: f64, t9003: f64, t9150: f64, t9162: f64, t9367: f64) -> (f64, f64) {
    let t38089 = 0.34694512752820797848e1_f64 * t32003 * t38086 * t36479;
    let t38092 = t394 * t2385;
    let t38104 = t7885 * t2132 * t2385 * t864;
    let t38108 = 0.8673628188205199462e0_f64 * t38073 - 0.8673628188205199462e0_f64 * t9003 * t8119 - 0.65854491829355115987e0_f64 * t38077 + 0.17347256376410398924e1_f64 * t33034 - 0.52041769129231196772e1_f64 * t7912 * t9150 + t33037 - t38085 + t38089 - 0.17347256376410398924e1_f64 * t31965 * t9162 - 0.17347256376410398924e1_f64 * t7931 * t38092 * t7934 + 0.17347256376410398924e1_f64 * t2146 * t2147 * t9367 * t463 + 0.17347256376410398924e1_f64 * t9003 * t8069 - 0.26020884564615598386e1_f64 * t38104 + 0.26341796731742046394e1_f64 * t33047 + 0.34694512752820797848e1_f64 * t33053;
    (t38092, t38108)
}
