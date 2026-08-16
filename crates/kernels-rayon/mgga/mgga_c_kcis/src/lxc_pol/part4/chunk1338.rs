//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1338/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1338(t17118: f64, t17153: f64, t17196: f64, t17242: f64, t1441: f64, t1951: f64, t1962: f64, t4016: f64, t11918: f64, t11947: f64, t11949: f64, t12084: f64, t1360: f64, t1404: f64, t1455: f64, t16360: f64, t17065: f64, t17066: f64, t17069: f64, t17073: f64, t17076: f64, t17079: f64, t17083: f64, t17088: f64, t1924: f64, t1979: f64, t3951: f64, t4023: f64, t4106: f64, t486: f64, t510: f64, t5623: f64, t5867: f64) -> (f64, f64) {
    let t17244 = t17118 + t17153 + t17196 + t17242;
    let t17248 = t1441 * t1951;
    let t17250 = t4016 * t1962;
    let t17252 = -t17065 + 0.93706135855523581992e-2_f64 * t1404 * t17066 + 0.46853067927761790996e-2_f64 * t1404 * t17069 + 0.28111840756657074598e-1_f64 * t510 * t17073 + 0.14055920378328537299e-1_f64 * t510 * t17076 - 0.93706135855523581992e-2_f64 * t4023 * t17079 - 0.56223681513314149196e-1_f64 * t510 * t17083 - 0.14055920378328537299e-1_f64 * t11918 - 0.46853067927761790996e-2_f64 * t11947 - t12084 - 0.18741227171104716398e-1_f64 * t17088 * t16360 - t1924 * t4106 - 2.0_f64 * t1360 * t5867 - 2.0_f64 * t5623 * t1455 - t486 * t17244 - 0.93706135855523581992e-2_f64 * t11949 - t3951 * t1979 - 0.46853067927761790996e-2_f64 * t17248 - 0.93706135855523581992e-2_f64 * t17250;
    (t17244, t17252)
}
