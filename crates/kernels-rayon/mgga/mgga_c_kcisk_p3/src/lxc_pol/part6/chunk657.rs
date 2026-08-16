//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 657/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk657(t1974: f64, t9124: f64, t5400: f64, t9108: f64, t1685: f64, t8590: f64, t8607: f64, t4790: f64, t1966: f64, t1979: f64, t2605: f64, t2609: f64, t5373: f64, t5398: f64, t5408: f64, t5415: f64, t7467: f64, t7498: f64, t764: f64, t8546: f64, t8548: f64, t8552: f64, t8576: f64, t8579: f64, t8585: f64, t9103: f64, t9109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9125 = t9124 * t1974;
    let t9128 = t9108 * t5400;
    let t9134 = t8590 * t1685;
    let t9137 = t8607 * t1685;
    let t9140 = t8590 * t4790;
    let t9143 = -0.3109e-1_f64 * t9103 * t764 + 2.0_f64 * t7467 * t2605 - 2.0_f64 * t5373 * t9109 + 1.0_f64 * t1966 * t9125 + 0.32164683177870697974e2_f64 * t5398 * t9128 + t8546 - t8548 + t8552 - t8576 - t8579 - 0.19751789702565206229e-1_f64 * t8585 + 0.11696446794910408142e1_f64 * t7498 * t2609 - 0.11696446794910408142e1_f64 * t5408 * t9134 + 0.58482233974552040708e0_f64 * t1979 * t9137 + 0.17315755899375863299e2_f64 * t5415 * t9140;
    (t9125, t9128, t9134, t9137, t9140, t9143)
}
