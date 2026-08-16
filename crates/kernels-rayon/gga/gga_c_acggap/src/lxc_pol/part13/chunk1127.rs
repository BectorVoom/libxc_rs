//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1127/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1127(t35479: f64, t35436: f64, t35439: f64, t35442: f64, t35445: f64, t35448: f64, t35449: f64, t35452: f64, t35454: f64, t35456: f64, t35459: f64, t35460: f64, t35464: f64, t35467: f64, t35469: f64, t35471: f64, t35476: f64) -> f64 {
    let t35480 = 0.28582678745379824648e-3_f64 * t35479;
    let t35481 = -0.80031500487063509016e-1_f64 * t35436 + t35439 / 24.0_f64 + t35442 / 24.0_f64 + 0.1528125e-1_f64 * t35445 + t35448 + 0.34299214494455789578e-2_f64 * t35449 - t35452 - 0.10718504529517434243e-3_f64 * t35454 + 0.10718504529517434243e-2_f64 * t35456 + t35459 + 0.13719685797782315831e-1_f64 * t35460 - 0.23586069217203114051e-2_f64 * t35464 + 0.10289764348336736873e-1_f64 * t35467 - 0.51448821741683684366e-2_f64 * t35469 + 0.95275595817932748827e-3_f64 * t35471 + t35476 + t35480;
    t35481
}
