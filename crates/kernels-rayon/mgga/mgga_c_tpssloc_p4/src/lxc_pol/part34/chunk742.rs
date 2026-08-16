//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 742/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk742(t3062: f64, t820: f64, t10402: f64, t3200: f64, t3051: f64, t121: f64, t3061: f64, t1008: f64, t349: f64, t1011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10408 = t820 * t3062;
    let t10413 = t3200 * t10402;
    let t10422 = t820 * t3051;
    let t10457 = t121 * t3061;
    let t10468 = t1008 * t1008;
    let t10469 = 1.0_f64 / t10468;
    let t10470 = t349 * t10469;
    let t10471 = t1011 * t1011;
    (t10408, t10413, t10422, t10457, t10469, t10470, t10471)
}
