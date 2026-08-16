//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1310/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1310<F: Float>(t1394: F, t27364: F, t29269: F, t18431: F, t4163: F, t7923: F, t29273: F, t4153: F, t102431: F, t102438: F, t102441: F, t102444: F, t102447: F, t27567: F, t28701: F, t28708: F, t29575: F, t94931: F, t98994: F, t99331: F, t99452: F) -> (F, F, F, F) {
    let t102450 = t1394 * t27364 * t29269;
    let t102454 = t1394 * t7923 * t4163 * t18431;
    let t102457 = t4153 * t27364 * t29273;
    let t102459 = -F::cast_from(0.185671721767578125e-4_f64) * t98994 * t28708 - t99452 - F::cast_from(0.15476481481481481481e-2_f64) * t102431 - F::cast_from(0.61782407407407407407e-3_f64) * t99331 * t28701 + F::cast_from(0.30918233506944444444e-4_f64) * t94931 * t29575 + F::cast_from(0.30918233506944444444e-4_f64) * t27567 * t102438 + F::cast_from(0.12897067901234567901e-2_f64) * t102441 + F::cast_from(0.23214722222222222222e-2_f64) * t102444 - F::cast_from(0.46429444444444444444e-2_f64) * t102447 + F::cast_from(0.11607361111111111111e-2_f64) * t102450 + F::cast_from(0.11607361111111111111e-2_f64) * t102454 + F::cast_from(0.19345601851851851852e-2_f64) * t102457;
    (t102450, t102454, t102457, t102459)
}
