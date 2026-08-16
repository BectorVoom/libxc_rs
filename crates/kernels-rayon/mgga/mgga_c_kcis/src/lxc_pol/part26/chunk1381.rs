//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1381/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1381(t102674: f64, t102678: f64, t102681: f64, t102684: f64, t102687: f64, t102694: f64, t102698: f64, t102701: f64, t2239: f64, t23157: f64, t28403: f64, t29404: f64, t3964: f64, t7916: f64, t8151: f64, t98874: f64) -> f64 {
    let t103731 = 0.16581944444444444444e-2_f64 * t102674 - 0.67960648148148148147e-2_f64 * t3964 * t23157 * t2239 - 0.37069444444444444444e-2_f64 * t8151 * t28403 + 0.67960648148148148147e-2_f64 * t29404 * t7916 + 0.13265555555555555555e-1_f64 * t102678 - 0.82376543209876543213e-3_f64 * t98874 - 0.55273148148148148147e-3_f64 * t102681 + 0.11054629629629629629e-2_f64 * t102684 - 0.33163888888888888888e-2_f64 * t102687 + 0.11054629629629629629e-2_f64 * t102694 + 0.22109259259259259258e-2_f64 * t102698 - 0.44218518518518518516e-2_f64 * t102701;
    t103731
}
