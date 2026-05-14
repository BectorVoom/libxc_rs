//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 969/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk969<F: Float>(t1203: F, t1212: F, t19691: F, t13064: F, t2105: F, t4479: F, t3696: F, t5788: F, t1543: F, t45: F, t5761: F, t1201: F, t1213: F, t19594: F, t19602: F, t19604: F, t19606: F, t19609: F, t19612: F, t19615: F, t19619: F, t19622: F, t19625: F, t19644: F, t3692: F, t3699: F, t5765: F, t5771: F) -> (F, F, F, F) {
    let t19693 = t1203 * t19691 * t1212;
    let t19698 = t13064 * t2105;
    let t19699 = t19698 * t4479;
    let t19702 = t3696 * t5788;
    let t19703 = t19702 * t1543;
    let t19706 = t45 * t5761;
    let t19709 = -0.17315755899375863299e2 * t1201 * t19594 + 0.11696446794910408142e1 * t5765 * t3699 + t19602 - t19604 + t19606 - t19609 - t19612 - t19615 + t19619 + t19622 + t19625 - t19644 - 0.58482233974552040708e0 * t1201 * t19693 + 0.23392893589820816284e1 * t3692 * t5771 + 0.1038945353962551798e3 * t1201 * t19699 + 0.23392893589820816284e1 * t1201 * t19703 - 0.11696446794910408142e1 * t19706 * t1213;
    (t19693, t19699, t19703, t19709)
}
