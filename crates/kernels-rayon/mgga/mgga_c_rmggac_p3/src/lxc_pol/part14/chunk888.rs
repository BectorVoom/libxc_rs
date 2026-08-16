//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 888/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk888(t321: f64, t3351: f64, t511: f64, t7248: f64, t8502: f64, t333: f64, t7231: f64, t880: f64, t2339: f64, t638: f64, t7184: f64, t7255: f64, t8427: f64) -> (f64, f64, f64, f64) {
    let t39379 = t3351 * t7248 * t511 * t8502 * t321;
    let t39384 = t3351 * t7231 * t880 * t8502 * t333;
    let t39388 = t638 * t7184 * t2339;
    let t39390 = t7255 * t8427;
    (t39379, t39384, t39388, t39390)
}
