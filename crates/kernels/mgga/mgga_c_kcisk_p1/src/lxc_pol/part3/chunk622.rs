//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 622/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk622<F: Float>(t1685: F, t4781: F, t4787: F, t591: F, t4762: F, t4790: F, t1966: F, t1975: F, t1979: F, t1980: F, t4698: F, t4701: F, t4708: F, t4739: F, t4747: F, t4754: F, t5365: F, t5368: F, t5373: F, t5375: F, t5393: F, t5398: F, t5401: F, t5405: F, t5408: F, t5409: F, t764: F) -> (F, F, F, F) {
    let t5412 = t4781 * t1685;
    let t5415 = t591 * t4787;
    let t5416 = t4762 * t4790;
    let t5419 = -F::new(0.3109e-1) * t5365 * t764 + F::new(2.0) * t5368 * t1975 - F::new(2.0) * t5373 * t5375 + F::new(1.0) * t1966 * t5393 + F::cast_from(0.32164683177870697974e2_f64) * t5398 * t5401 + t4698 - t4701 + t4708 - t4739 - t4747 - F::cast_from(0.19751789702565206229e-1_f64) * t4754 + F::cast_from(0.11696446794910408142e1_f64) * t5405 * t1980 - F::cast_from(0.11696446794910408142e1_f64) * t5408 * t5409 + F::cast_from(0.58482233974552040708e0_f64) * t1979 * t5412 + F::cast_from(0.17315755899375863299e2_f64) * t5415 * t5416;
    (t5412, t5415, t5416, t5419)
}
