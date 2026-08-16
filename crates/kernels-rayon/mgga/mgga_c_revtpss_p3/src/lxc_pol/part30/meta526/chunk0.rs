//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1939/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1939(t4181: f64, t603: f64, t4187: f64, t1493: f64, t644: f64, t77: f64, t4173: f64, t607: f64, t7705: f64, t1497: f64, t1927: f64, t1470: f64, t2247: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28116 = t603 * t4181;
    let t28119 = t603 * t4187;
    let t28133 = t77 * t1493 * t644;
    let t28141 = t4173 * t607;
    let t28147 = t77 * t7705 * t644;
    let t28150 = t1927 * t1497;
    let t28154 = t2247 * t1470;
    (t28116, t28119, t28133, t28141, t28147, t28150, t28154)
}
