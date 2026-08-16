//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 569/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk569(t3335: f64, t3340: f64, t3344: f64, t3349: f64, t3356: f64, t3359: f64, t3363: f64, t3366: f64, t3370: f64, t3426: f64, t3430: f64, t3433: f64) -> f64 {
    let t3685 = 0.5e0_f64 * t3335 - 0.125e0_f64 * t3340 + 0.625e-1_f64 * t3344 - 0.44965277777777777777e-2_f64 * t3349 - 0.34173611111111111111e0_f64 * t3356 + 0.14388888888888888889e0_f64 * t3359 + 0.91666666666666666667e0_f64 * t3363 - 0.33333333333333333334e0_f64 * t3366 - 0.101171875e-1_f64 * t3370 + 0.9375e-1_f64 * t3426 - 0.20833333333333333333e-1_f64 * t3430 - 0.10791666666666666667e0_f64 * t3433;
    t3685
}
