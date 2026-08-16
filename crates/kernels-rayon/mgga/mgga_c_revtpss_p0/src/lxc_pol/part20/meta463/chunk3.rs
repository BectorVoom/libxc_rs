//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1763/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1763(t9905: f64, t9976: f64, t221: f64, t9984: f64, t3978: f64, t9921: f64, t3926: f64, t9909: f64, t3930: f64, t9901: f64, t2661: f64, t5675: f64, t9929: f64, t9934: f64) -> (f64, f64, f64, f64, f64) {
    let t47298 = t9976 * t9905;
    let t47300 = t221 * t9984;
    let t47302 = t3978 * t9921 * t47300;
    let t47304 = t9909 * t3926;
    let t47306 = t3930 * t9901;
    let t47318 = t2661 * t9934 * t9929 * t5675;
    (t47298, t47302, t47304, t47306, t47318)
}
