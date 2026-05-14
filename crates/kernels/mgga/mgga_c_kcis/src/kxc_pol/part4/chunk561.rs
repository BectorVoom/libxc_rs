//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 561/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk561<F: Float>(t3032: F, t3034: F, t2932: F, t2935: F, t2942: F, t2983: F, t2991: F, t2998: F, t3001: F, t3008: F, t3027: F, t45: F, t960: F, t972: F) -> (F, F) {
    let t3035 = t3032 * t3034;
    let t3038 = -t2932 + t2935 - t2942 + t2983 + t2991 + 0.19751789702565206229e-1 * t45 * t2998 - 0.11696446794910408142e1 * t3001 * t972 + 0.11696446794910408142e1 * t960 * t3008 - 0.58482233974552040708e0 * t960 * t3027 - 0.17315755899375863299e2 * t960 * t3035;
    (t3035, t3038)
}
