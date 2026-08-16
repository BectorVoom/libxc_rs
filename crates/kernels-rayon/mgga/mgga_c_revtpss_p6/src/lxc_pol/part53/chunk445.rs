//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 445/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk445(t2231: f64, t27: f64, t21: f64, t25: f64, t599: f64, t602: f64, t89: f64, t90: f64, t29: f64) -> (f64, f64, f64, f64, f64) {
    let t2233 = 30.0_f64 * t2231 * t27;
    let t2236 = t21 * t21;
    let t2237 = 1.0_f64 / t2236;
    let t2239 = 42.0_f64 * t25 * t2237;
    let t2242 = t599 * t602;
    let t2246 = 1.0_f64 / t90 / t89;
    let t2247 = t29 * t2246;
    (t2233, t2239, t2242, t2246, t2247)
}
