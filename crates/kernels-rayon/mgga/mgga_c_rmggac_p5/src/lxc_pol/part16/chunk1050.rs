//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1050/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1050(t1997: f64, t47871: f64, t2004: f64, t45561: f64, t39277: f64, t9118: f64, t1910: f64, t3351: f64, t352: f64, t515: f64, t7231: f64, t7720: f64, t9790: f64) -> (f64, f64, f64, f64, f64) {
    let t47872 = t47871 * t1997;
    let t47874 = t45561 * t2004;
    let t47876 = t39277 * t9118;
    let t47881 = t3351 * t7231 * t515 * t1910 * t352;
    let t47883 = t7720 * t9790;
    (t47872, t47874, t47876, t47881, t47883)
}
