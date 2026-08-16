//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1317/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1317(t5527: f64, t857: f64, t23204: f64, t28298: f64, t81640: f64, t225: f64, t28442: f64, t5544: f64, t23164: f64, t28276: f64, t28342: f64, t81979: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98224 = t857 * t5527;
    let t98237 = t81640 * t23204 * t28298;
    let t98239 = t28442 * t225;
    let t98253 = t857 * t5544;
    let t98322 = t23164 * t23204 * t28276;
    let t98330 = t81979 * t28342;
    (t98224, t98237, t98239, t98253, t98322, t98330)
}
