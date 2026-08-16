//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1069/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1069(t23956: f64, t24446: f64, t3: f64, t112: f64, t7222: f64, t111: f64, t2098: f64, t671: f64, t7056: f64, t2039: f64, t2363: f64, t12521: f64, t12524: f64, t1401: f64, t16535: f64, t2319: f64, t23917: f64, t3938: f64, t3941: f64, t577: f64, t7230: f64, t7235: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24447 = t23956 + t24446;
    let t24448 = t3 * t24447;
    let t24462 = t7222 * t112;
    let t24465 = t2098 * t111;
    let t24478 = t7056 * t671;
    let t24481 = t2039 * t2363;
    let t24486 = 0.45e1_f64 * t24447 * t577 + 27.0_f64 * t24462 * t671 + 27.0_f64 * t24465 * t2319 + 0.135e2_f64 * t7230 * t2363 + 0.135e2_f64 * t12521 * t2039 + 54.0_f64 * t12524 * t7235 + 27.0_f64 * t3938 * t7056 + 27.0_f64 * t16535 * t2039 + 54.0_f64 * t3941 * t24478 + 27.0_f64 * t3941 * t24481 + 0.135e2_f64 * t1401 * t23917;
    (t24447, t24448, t24462, t24465, t24478, t24481, t24486)
}
