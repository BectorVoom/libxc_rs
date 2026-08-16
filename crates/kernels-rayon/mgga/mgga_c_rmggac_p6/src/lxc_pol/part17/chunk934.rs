//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 934/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk934(t1763: f64, t3351: f64, t498: f64, t7231: f64, t875: f64, t30800: f64, t3352: f64, t1971: f64, t30490: f64, t7262: f64, t352: f64, t4617: f64) -> (f64, f64, f64, f64, f64) {
    let t45546 = t3351 * t7231 * t875 * t1763 * t498;
    let t45550 = t3351 * t3352 * t875 * t30800;
    let t45554 = t3351 * t1971 * t7262 * t30490;
    let t45556 = t1763 * t352;
    let t45559 = t3351 * t1971 * t4617 * t45556;
    (t45546, t45550, t45554, t45556, t45559)
}
