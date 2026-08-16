//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2220/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2220(t23204: f64, t28298: f64, t81640: f64, t225: f64, t28442: f64, t22986: f64, t23270: f64, t25191: f64, t4300: f64, t25192: f64, t86873: f64, t5544: f64, t857: f64) -> (f64, f64, f64, f64, f64) {
    let t98237 = t81640 * t23204 * t28298;
    let t98239 = t28442 * t225;
    let t98248 = t22986 * t23270 * t25191 * t4300;
    let t98251 = t22986 * t86873 * t25192;
    let t98253 = t857 * t5544;
    (t98237, t98239, t98248, t98251, t98253)
}
