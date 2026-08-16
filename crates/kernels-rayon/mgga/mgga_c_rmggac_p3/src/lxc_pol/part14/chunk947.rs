//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 947/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk947(t40262: f64, t16504: f64, t34975: f64, t552: f64, t7455: f64, t16503: f64, t34962: f64, t7461: f64, t22971: f64, t7467: f64, t1965: f64, t1967: f64, t28: f64, t8511: f64) -> (f64, f64, f64, f64, f64) {
    let t40263 = 0.39726959900411316772e-4_f64 * t40262;
    let t40266 = t34975 * t16504 * t552 * t7455;
    let t40270 = t16503 * t34962 * t552 * t7461;
    let t40274 = t16503 * t22971 * t552 * t7467;
    let t40278 = t8511 * t1965 * t1967 * t28;
    (t40263, t40266, t40270, t40274, t40278)
}
