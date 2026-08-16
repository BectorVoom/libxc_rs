//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1214/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1214(t3738: f64, t5935: f64, t1528: f64, t1928: f64, t7953: f64, t17457: f64, t27544: f64, t1468: f64, t17703: f64, t17467: f64, t4254: f64, t572: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97782 = t3738 * t5935;
    let t97784 = t1528 * t1928;
    let t97785 = t97784 * t7953;
    let t97787 = t27544 * t17457;
    let t97789 = t1468 * t17703;
    let t97791 = t27544 * t17467;
    let t97793 = t4254 * t572;
    (t97782, t97785, t97787, t97789, t97791, t97793)
}
