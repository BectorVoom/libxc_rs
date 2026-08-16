//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1167/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1167(t12254: f64, t12257: f64, t141: f64, t1146: f64, t2439: f64, t3424: f64, t698: f64, t3421: f64, t3361: f64, t57: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12258 = t12254 * t12257;
    let t12259 = t141 * t12258;
    let t12261 = t2439 * t1146;
    let t12263 = t698 * t3424;
    let t12265 = t698 * t3421;
    let t12267 = t3361 * t57;
    let t12268 = 1.0_f64 / t12267;
    (t12258, t12259, t12261, t12263, t12265, t12267, t12268)
}
