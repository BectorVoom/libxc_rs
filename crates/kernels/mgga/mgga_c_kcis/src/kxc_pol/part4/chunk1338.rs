//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1338/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1338<F: Float>(t17118: F, t17153: F, t17196: F, t17242: F, t1441: F, t1951: F, t1962: F, t4016: F, t11918: F, t11947: F, t11949: F, t12084: F, t1360: F, t1404: F, t1455: F, t16360: F, t17065: F, t17066: F, t17069: F, t17073: F, t17076: F, t17079: F, t17083: F, t17088: F, t1924: F, t1979: F, t3951: F, t4023: F, t4106: F, t486: F, t510: F, t5623: F, t5867: F) -> (F, F) {
    let t17244 = t17118 + t17153 + t17196 + t17242;
    let t17248 = t1441 * t1951;
    let t17250 = t4016 * t1962;
    let t17252 = -t17065 + F::cast_from(0.93706135855523581992e-2_f64) * t1404 * t17066 + F::cast_from(0.46853067927761790996e-2_f64) * t1404 * t17069 + F::cast_from(0.28111840756657074598e-1_f64) * t510 * t17073 + F::cast_from(0.14055920378328537299e-1_f64) * t510 * t17076 - F::cast_from(0.93706135855523581992e-2_f64) * t4023 * t17079 - F::cast_from(0.56223681513314149196e-1_f64) * t510 * t17083 - F::cast_from(0.14055920378328537299e-1_f64) * t11918 - F::cast_from(0.46853067927761790996e-2_f64) * t11947 - t12084 - F::cast_from(0.18741227171104716398e-1_f64) * t17088 * t16360 - t1924 * t4106 - F::new(2.0) * t1360 * t5867 - F::new(2.0) * t5623 * t1455 - t486 * t17244 - F::cast_from(0.93706135855523581992e-2_f64) * t11949 - t3951 * t1979 - F::cast_from(0.46853067927761790996e-2_f64) * t17248 - F::cast_from(0.93706135855523581992e-2_f64) * t17250;
    (t17244, t17252)
}
