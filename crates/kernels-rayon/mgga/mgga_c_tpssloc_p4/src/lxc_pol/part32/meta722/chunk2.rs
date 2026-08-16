//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2301/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2301(t103345: f64, t2122: f64, t24574: f64, t29674: f64, t29750: f64, t85853: f64, t1011: f64, t6218: f64, t225: f64, t29624: f64, t29614: f64, t103223: f64, t19189: f64, t24589: f64, t24788: f64, t24812: f64, t24833: f64, t27461: f64, t27473: f64, t27489: f64, t27516: f64, t27553: f64, t29740: f64, t29744: f64, t4978: f64, t7364: f64, t7373: f64, t7375: f64, t7376: f64, t94784: f64, t94787: f64) -> (f64, f64, f64, f64) {
    let t103490 = t2122 * t103345;
    let t103494 = t24574 * t29674;
    let t103507 = t85853 * t29750;
    let t103515 = t6218 * t1011;
    let t103520 = t29624 * t225;
    let t103528 = t29614 * t225;
    let t103538 = 0.54831135561607547884e-2_f64 * t24589 * t27516 * t27473 + 0.54831135561607547883e-2_f64 * t103507 - 0.36554090374405031923e-2_f64 * t94784 - 0.19495514866349350359e-1_f64 * t103223 * t27553 + t94787 - 0.82246703342411321825e-2_f64 * t7373 * t24833 * t29744 + 0.16449340668482264365e-1_f64 * t24812 * t27489 * t103515 * t4978 + 0.27415567780803773942e-2_f64 * t24589 * t103520 * t7364 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t19189 * t7376 + 0.27415567780803773942e-2_f64 * t24589 * t103528 * t7364 + 0.54831135561607547884e-2_f64 * t24589 * t24788 * t29740 + 0.54831135561607547883e-2_f64 * t24589 * t27516 * t27461;
    (t103490, t103494, t103515, t103538)
}
