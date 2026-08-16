//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 881/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk881(t25: f64, t265: f64, t394: f64, t31477: f64, t1877: f64, t24191: f64, t24339: f64, t2522: f64, t26756: f64, t30767: f64, t31430: f64, t31434: f64, t31442: f64, t31449: f64, t31451: f64, t40: f64, t606: f64, t607: f64, t6542: f64, t6671: f64, t7114: f64, t8566: f64, t8569: f64, t8580: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t31478 = piecewise3(t395, 0.0_f64, t31477);
    let t31483 = piecewise3(t115, 3.0_f64 / 2.0_f64 * t2522 * t8566 * t6542 + t1877 * t31430 * t25 / 2.0_f64 - t1877 * t31434 * t6671 / 2.0_f64 + t1877 * t8566 * t606 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t31442 - t1877 * t24339 * t8569 / 2.0_f64 + t26756 * t31449 - t1877 * t7114 * t31451 / 2.0_f64 - t1877 * t7114 * t30767 / 2.0_f64, t31478 * t40 / 2.0_f64 + t8580 * t607 / 2.0_f64);
    (t31478, t31483)
}
