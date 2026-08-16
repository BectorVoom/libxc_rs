//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1251/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1251(t13602: f64, t13566: f64, t2932: f64, t4471: f64, t300: f64, t4446: f64, t3053: f64, t4644: f64, t10422: f64, t4578: f64, t3070: f64, t1603: f64, t3030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14354 = 0.61805555555555555556e-2_f64 * t13602;
    let t14409 = 0.2283111111111111111e-1_f64 * t13566;
    let t14410 = 0.11415555555555555555e-1_f64 * t13602;
    let t14459 = t4471 * t2932;
    let t14473 = t300 * t4446;
    let t14495 = t4644 * t3053 / 3456.0_f64;
    let t14501 = t10422 * t4578;
    let t14503 = t3070 * t14501 / 3456.0_f64;
    let t14506 = t1603 * t3030;
    (t14354, t14409, t14410, t14459, t14473, t14495, t14503, t14506)
}
