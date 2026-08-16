//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 773/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk773(t1484: f64, t232: f64, t2645: f64, t4181: f64, t4212: f64, t185: f64, t5398: f64, t707: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2486: f64, t2518: f64, t2530: f64, t2537: f64, t2665: f64, t5497: f64, t5498: f64, t5501: f64, t5506: f64, t5521: f64, t5524: f64, t5525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5591 = t232 * t1484;
    let t5593 = t2645 * t4181 * t5591;
    let t5596 = 0.36622894612013090108e-3_f64 * t4212;
    let t5597 = t185 * t5398;
    let t5599 = 4.0_f64 * t707 * t5597;
    let t5600 = t2373 + t5524 + t5521 + t5498 + t2377 + t5497 - t2486 - t5596 - t5525 + t5506 + t2518 + t2408 + t2417 + t5501 - t2530 - t2537 - t2426 + t2665 - t2423 + t5599;
    (t5591, t5593, t5596, t5597, t5599, t5600)
}
