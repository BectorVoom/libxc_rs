//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 778/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk778(t1114: f64, t4231: f64, t3931: f64, t1569: f64, t943: f64, t1108: f64, t938: f64, t1120: f64, t1571: f64, t357: f64, t339: f64, t454: f64) -> (f64, f64, f64, f64, f64) {
    let t4252 = t4231 * t1114;
    let t4253 = t3931 * t4252;
    let t4256 = t1569 * t943;
    let t4258 = t938 * t1108 * t4256;
    let t4261 = t1571 * t1120;
    let t4263 = t1569 * t357;
    let t4265 = t339 * t454 * t4263;
    (t4252, t4253, t4258, t4261, t4265)
}
