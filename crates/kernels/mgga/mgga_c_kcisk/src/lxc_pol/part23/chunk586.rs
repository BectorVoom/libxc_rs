//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 586/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk586<F: Float>(t1254: F, t4075: F, t1232: F, t346: F, t360: F, t4032: F, t4007: F, t4011: F, t4015: F, t4018: F, t4021: F, t1260: F, t45: F, t1265: F, t370: F, t1273: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4076 = t4075 * t1254;
    let t4079 = t1232 * t1232;
    let t4080 = 1.0 / t4079;
    let t4081 = t346 * t4080;
    let t4082 = t360 * t360;
    let t4083 = 1.0 / t4082;
    let t4084 = t4032 * t4083;
    let t4087 = 0.12361111111111111111e-1 * t4007;
    let t4092 = t4087 + 0.61805555555555555556e-2 * t4011 - 0.61805555555555555555e-2 * t4015 + 0.18541666666666666667e-1 * t4018 - 0.92708333333333333333e-2 * t4021;
    let t4096 = t45 * t1260;
    let t4099 = t1265 * t370;
    let t4100 = 1.0 / t4099;
    let t4101 = t1273 * t1273;
    (t4076, t4079, t4080, t4081, t4082, t4083, t4084, t4087, t4092, t4096, t4100, t4101)
}
