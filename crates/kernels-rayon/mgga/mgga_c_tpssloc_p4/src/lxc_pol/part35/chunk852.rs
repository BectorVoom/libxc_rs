//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 852/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk852(t2770: f64, t343: f64, t2775: f64, t2769: f64, t40: f64, t344: f64, t241: f64, t625: f64, t281: f64, t283: f64, t2978: f64, t340: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10236 = t343 * t2770;
    let t10254 = t343 * t2775;
    let t10276 = t2769 * t40;
    let t10277 = 1.0_f64 / t10276;
    let t10278 = t344 * t10277;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    let t10295 = 20.0_f64 / 27.0_f64 * t10294;
    let t10304 = t241 * t2978;
    let t10335 = t63 * t340;
    (t10236, t10254, t10277, t10278, t10292, t10294, t10295, t10304, t10335)
}
