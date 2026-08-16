//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 926/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk926(t10292: f64, t281: f64, t283: f64, t2403: f64, t909: f64, t241: f64, t2978: f64, t2967: f64, t964: f64, t340: f64, t63: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10294 = t281 * t10292 * t283;
    let t10295 = 20.0_f64 / 27.0_f64 * t10294;
    let t10296 = t2403 * t909;
    let t10304 = t241 * t2978;
    let t10333 = t964 * t2967;
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    (t10294, t10295, t10296, t10304, t10333, t10335, t10336)
}
