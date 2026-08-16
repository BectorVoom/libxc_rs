//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1323/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1323(t1014: f64, t29372: f64, t27475: f64, t303: f64, t6932: f64, t1459: f64, t6923: f64, t1394: f64, t20878: f64, t7923: f64, t20883: f64, t21510: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102674 = t1014 * t29372;
    let t102678 = t303 * t27475 * t6932;
    let t102681 = t303 * t1459 * t6923;
    let t102684 = t1394 * t7923 * t20878;
    let t102687 = t1394 * t7923 * t20883;
    let t102694 = t1394 * t7923 * t21510;
    (t102674, t102678, t102681, t102684, t102687, t102694)
}
