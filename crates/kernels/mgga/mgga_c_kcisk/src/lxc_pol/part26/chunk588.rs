//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 588/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk588<F: Float>(t1203: F, t1212: F, t5788: F, t2105: F, t3722: F, t1210: F, t3725: F, t1201: F, t1213: F, t2107: F, t3692: F, t45: F, t5714: F, t5717: F, t5719: F, t5722: F, t5751: F, t5755: F, t5762: F, t5765: F, t5771: F) -> (F, F, F, F, F) {
    let t5790 = t1203 * t5788 * t1212;
    let t5793 = t3722 * t2105;
    let t5794 = t3725 * t1210;
    let t5795 = t5793 * t5794;
    let t5798 = -t5714 + t5717 + t5719 - t5722 + t5751 + t5755 + 0.19751789702565206229e-1 * t45 * t5762 - 0.58482233974552040708e0 * t5765 * t1213 - 0.58482233974552040708e0 * t3692 * t2107 + 0.11696446794910408142e1 * t1201 * t5771 - 0.58482233974552040708e0 * t1201 * t5790 - 0.17315755899375863299e2 * t1201 * t5795;
    (t5790, t5793, t5794, t5795, t5798)
}
