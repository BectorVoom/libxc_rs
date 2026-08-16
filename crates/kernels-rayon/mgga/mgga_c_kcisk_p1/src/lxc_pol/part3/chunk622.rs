//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 622/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk622(t1685: f64, t4781: f64, t4787: f64, t591: f64, t4762: f64, t4790: f64, t1966: f64, t1975: f64, t1979: f64, t1980: f64, t4698: f64, t4701: f64, t4708: f64, t4739: f64, t4747: f64, t4754: f64, t5365: f64, t5368: f64, t5373: f64, t5375: f64, t5393: f64, t5398: f64, t5401: f64, t5405: f64, t5408: f64, t5409: f64, t764: f64) -> (f64, f64, f64, f64) {
    let t5412 = t4781 * t1685;
    let t5415 = t591 * t4787;
    let t5416 = t4762 * t4790;
    let t5419 = -0.3109e-1_f64 * t5365 * t764 + 2.0_f64 * t5368 * t1975 - 2.0_f64 * t5373 * t5375 + 1.0_f64 * t1966 * t5393 + 0.32164683177870697974e2_f64 * t5398 * t5401 + t4698 - t4701 + t4708 - t4739 - t4747 - 0.19751789702565206229e-1_f64 * t4754 + 0.11696446794910408142e1_f64 * t5405 * t1980 - 0.11696446794910408142e1_f64 * t5408 * t5409 + 0.58482233974552040708e0_f64 * t1979 * t5412 + 0.17315755899375863299e2_f64 * t5415 * t5416;
    (t5412, t5415, t5416, t5419)
}
