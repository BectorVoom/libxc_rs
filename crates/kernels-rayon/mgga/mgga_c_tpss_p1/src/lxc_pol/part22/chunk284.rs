//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 284/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk284(t294: f64, t305: f64, t843: f64, t869: f64, t872: f64, t877: f64, t886: f64, t892: f64, t896: f64, t905: f64, t309: f64) -> (f64, f64, f64) {
    let t909 = t294 * (-0.310907e-1_f64 * t872 * t305 + 1.0_f64 * t877 * t886 + t843 - t869 - 0.19751673498613801407e-1_f64 * t892 + 0.5848223622634646207e0_f64 * t896 * t905);
    let t911 = 0.19751673498613801407e-1_f64 * t294 * t892;
    let t912 = t294 * t309;
    (t909, t911, t912)
}
