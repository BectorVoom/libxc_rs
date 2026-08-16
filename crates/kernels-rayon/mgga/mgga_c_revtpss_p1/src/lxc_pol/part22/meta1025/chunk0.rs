//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3586/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3586(t20267: f64, t698: f64, t1145: f64, t141: f64, t68273: f64, t2258: f64, t6421: f64, t68269: f64, t20297: f64, t3417: f64, t20292: f64, t2251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t68312 = t698 * t20267;
    let t68315 = t141 * t1145 * t68273;
    let t68317 = t6421 * t2258;
    let t68319 = t141 * t1145 * t68317;
    let t68322 = t141 * t1145 * t68269;
    let t68324 = t20297 * t2258;
    let t68326 = t141 * t3417 * t68324;
    let t68328 = t20292 * t2251;
    (t68312, t68315, t68317, t68319, t68322, t68324, t68326, t68328)
}
