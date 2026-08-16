//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1576/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1576(t698: f64, t986: f64, t973: f64, t135: f64, t3010: f64, t241: f64, t625: f64, t281: f64, t283: f64, t2403: f64, t909: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10286 = t698 * t986;
    let t10287 = t973 * t10286;
    let t10289 = t135 * t3010;
    let t10290 = t973 * t10289;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    let t10295 = 20.0_f64 / 27.0_f64 * t10294;
    let t10296 = t2403 * t909;
    (t10286, t10287, t10290, t10292, t10294, t10295, t10296)
}
