//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 987/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk987(t19280: f64, t824: f64, t193: f64, t89: f64, t16579: f64, t792: f64, t666: f64, t5225: f64, t7640: f64, t4056: f64, t4129: f64, t2336: f64, t5221: f64) -> (f64, f64, f64, f64, f64) {
    let t19281 = t19280 * t824;
    let t19283 = t89 * t193 * t19281;
    let t19285 = t792 * t16579;
    let t19287 = t89 * t666 * t19285;
    let t19289 = t7640 * t5225;
    let t19290 = t19289 * t824;
    let t19292 = t89 * t193 * t19290;
    let t19293 = t4056 * t4129;
    let t19295 = t89 * t193 * t19293;
    let t19298 = t89 * t2336 * t5221;
    (t19283, t19287, t19292, t19295, t19298)
}
