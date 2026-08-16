//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 642/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk642(t1023: f64, t248: f64, t3101: f64, t1020: f64, t1041: f64, t1046: f64, t3039: f64, t3043: f64, t3048: f64, t3054: f64, t3057: f64, t3064: f64, t3070: f64, t3073: f64, t3078: f64, t3084: f64, t3089: f64, t3092: f64, t3094: f64, t3098: f64, t378: f64) -> (f64, f64, f64) {
    let t3103 = t248 * t3101 * t1023;
    let t3104 = t1020 * t3103;
    let t3106 = -t3039 * t3043 / 3072.0_f64 - t3048 * t1046 / 432.0_f64 + t3054 / 3456.0_f64 + t1041 * t3057 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t1041 * t3064 + t3070 * t3073 / 2304.0_f64 + t3078 * t378 / 3072.0_f64 - t3084 + 19.0_f64 / 1728.0_f64 * t3089 * t378 - t3092 / 432.0_f64 - t3094 * t378 / 288.0_f64 - t1041 * t3098 / 2304.0_f64 + t3104 / 2304.0_f64;
    (t3103, t3104, t3106)
}
