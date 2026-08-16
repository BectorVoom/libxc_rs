//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1646/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1646(t2382: f64, t4186: f64, t2615: f64, t4311: f64, t1469: f64, t2609: f64, t706: f64, t80: f64, t83: f64, t1568: f64, t785: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14416 = t2382 * t4186;
    let t14433 = 8.0_f64 * t4311 * t2615;
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14447 = t80 * t4186;
    let t14458 = t83 * t4186;
    let t14472 = t785 * t1568;
    let t14473 = t14472 * t780;
    (t14416, t14433, t14440, t14441, t14447, t14458, t14472, t14473)
}
