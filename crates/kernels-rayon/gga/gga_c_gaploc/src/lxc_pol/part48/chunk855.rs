//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 855/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk855(t209: f64, t44303: f64, t44351: f64, t44412: f64, t44452: f64, t44501: f64, t44545: f64, t44615: f64, t44666: f64, t37275: f64, t921: f64, t2497: f64, t3553: f64, t4349: f64) -> (f64, f64, f64) {
    let t44670 = (t44303 + t44351 + t44412 + t44452 + t44501 + t44545 + t44615 + t44666) * t209;
    let t44671 = t37275 * t921;
    let t44674 = 6.0_f64 * t4349 * t3553 * t2497;
    (t44670, t44671, t44674)
}
