//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1310/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1310(t1394: f64, t27364: f64, t29269: f64, t18431: f64, t4163: f64, t7923: f64, t29273: f64, t4153: f64, t102431: f64, t102438: f64, t102441: f64, t102444: f64, t102447: f64, t27567: f64, t28701: f64, t28708: f64, t29575: f64, t94931: f64, t98994: f64, t99331: f64, t99452: f64) -> (f64, f64, f64, f64) {
    let t102450 = t1394 * t27364 * t29269;
    let t102454 = t1394 * t7923 * t4163 * t18431;
    let t102457 = t4153 * t27364 * t29273;
    let t102459 = -0.185671721767578125e-4_f64 * t98994 * t28708 - t99452 - 0.15476481481481481481e-2_f64 * t102431 - 0.61782407407407407407e-3_f64 * t99331 * t28701 + 0.30918233506944444444e-4_f64 * t94931 * t29575 + 0.30918233506944444444e-4_f64 * t27567 * t102438 + 0.12897067901234567901e-2_f64 * t102441 + 0.23214722222222222222e-2_f64 * t102444 - 0.46429444444444444444e-2_f64 * t102447 + 0.11607361111111111111e-2_f64 * t102450 + 0.11607361111111111111e-2_f64 * t102454 + 0.19345601851851851852e-2_f64 * t102457;
    (t102450, t102454, t102457, t102459)
}
