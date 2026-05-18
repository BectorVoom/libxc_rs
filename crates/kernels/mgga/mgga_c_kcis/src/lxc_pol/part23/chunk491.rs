//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 491/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk491<F: Float>(t4072: F, t4105: F, t1360: F, t1404: F, t1455: F, t3951: F, t4018: F, t4019: F, t4021: F, t4023: F, t4024: F, t4028: F, t4031: F, t4036: F, t4039: F, t486: F, t510: F, t538: F) -> (F, F) {
    let t4106 = t4072 + t4105;
    let t4108 = t4018 + F::new(0.46853067927761790996e-2) * t4019 + F::new(0.93706135855523581992e-2) * t4021 + F::new(0.46853067927761790996e-2) * t4023 * t4024 + F::new(0.93706135855523581992e-2) * t1404 * t4028 - F::new(0.23426533963880895498e-2) * t1404 * t4031 + F::new(0.14055920378328537299e-1) * t510 * t4036 - F::new(0.46853067927761790996e-2) * t510 * t4039 - t3951 * t538 - F::new(2.0) * t1360 * t1455 - t486 * t4106;
    (t4106, t4108)
}
