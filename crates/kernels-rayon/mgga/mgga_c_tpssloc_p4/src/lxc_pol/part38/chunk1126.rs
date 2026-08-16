//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1126/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1126(t14032: f64, t3071: f64, t1616: f64, t2771: f64, t10408: f64, t1539: f64, t3121: f64, t3048: f64, t4571: f64, t10390: f64, t10891: f64, t10904: f64, t10937: f64, t10957: f64, t14006: f64, t14009: f64, t14012: f64, t14015: f64, t14018: f64, t14027: f64, t1622: f64, t3070: f64, t3098: f64, t4575: f64, t4596: f64, t4600: f64, t4644: f64, t973: f64) -> f64 {
    let t14033 = t3071 * t14032;
    let t14036 = t1616 * t2771;
    let t14037 = t10408 * t14036;
    let t14040 = t1539 * t3121;
    let t14041 = t3071 * t14040;
    let t14049 = t3048 * t4571 / 648.0_f64;
    let t14050 = -t973 * t14006 / 144.0_f64 - t973 * t14009 / 36.0_f64 + t973 * t14012 / 108.0_f64 + t973 * t14015 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t973 * t14018 - t10904 * t4596 / 144.0_f64 + t10891 * t4600 / 288.0_f64 + t14027 + t10390 * t4575 / 2304.0_f64 - t10937 * t4575 / 432.0_f64 + t3070 * t14033 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t3070 * t14037 + t3070 * t14041 / 4608.0_f64 - t4644 * t3098 / 2304.0_f64 + 19.0_f64 / 2592.0_f64 * t10957 * t1622 - t14049;
    t14050
}
