//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1054/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1054(t1502: f64, t16503: f64, t16504: f64, t552: f64, t10078: f64, t34761: f64, t34962: f64, t8420: f64, t1756: f64, t3351: f64, t498: f64, t515: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t47946 = t16503 * t16504 * t552 * t1502;
    let t47948 = t34761 * t10078;
    let t47952 = t16503 * t34962 * t552 * t8420;
    let t47957 = t3351 * t7231 * t515 * t1756 * t498;
    (t47946, t47948, t47952, t47957)
}
