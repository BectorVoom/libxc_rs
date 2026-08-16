//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 643/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk643<F: Float>(t1023: F, t248: F, t3101: F, t1020: F, t1041: F, t1046: F, t3039: F, t3043: F, t3048: F, t3054: F, t3057: F, t3064: F, t3070: F, t3073: F, t3078: F, t3084: F, t3089: F, t3092: F, t3094: F, t3098: F, t378: F) -> (F, F, F) {
    let t3103 = t248 * t3101 * t1023;
    let t3104 = t1020 * t3103;
    let t3106 = -t3039 * t3043 / F::cast_from(3072.0_f64) - t3048 * t1046 / F::cast_from(432.0_f64) + t3054 / F::cast_from(3456.0_f64) + t1041 * t3057 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1041 * t3064 + t3070 * t3073 / F::cast_from(2304.0_f64) + t3078 * t378 / F::cast_from(3072.0_f64) - t3084 + F::cast_from(19.0_f64) / F::cast_from(1728.0_f64) * t3089 * t378 - t3092 / F::cast_from(432.0_f64) - t3094 * t378 / F::cast_from(288.0_f64) - t1041 * t3098 / F::cast_from(2304.0_f64) + t3104 / F::cast_from(2304.0_f64);
    (t3103, t3104, t3106)
}
