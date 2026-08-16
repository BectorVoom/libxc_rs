//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1261/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1261(t225: f64, t9990: f64, t213: f64, t10605: f64, t162: f64, t10439: f64, t2394: f64, t262: f64, t10867: f64, t10871: f64, t2722: f64, t73: f64, t830: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    let t14325 = t10605 * t162;
    let t14330 = t10439 * t162;
    let t14375 = t2394 * t262;
    let t14545 = t225 * t10867;
    let t14546 = t213 * t14545;
    let t14547 = t10871 * t2722;
    let t14643 = t830 * t73;
    (t14192, t14193, t14325, t14330, t14375, t14545, t14546, t14547, t14643)
}
