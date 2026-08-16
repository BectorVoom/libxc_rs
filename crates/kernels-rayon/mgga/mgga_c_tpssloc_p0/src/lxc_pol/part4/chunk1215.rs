//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1215/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1215(t12346: f64, t12366: f64, t12429: f64, t1363: f64, t16233: f64, t16394: f64, t16400: f64, t19940: f64, t19942: f64, t19945: f64, t19951: f64, t19958: f64, t19962: f64, t19966: f64, t19972: f64, t19976: f64, t19981: f64, t19986: f64, t19991: f64, t19996: f64, t20000: f64, t20004: f64, t3803: f64, t5246: f64, t5259: f64, t6396: f64) -> f64 {
    let t20007 = -35.0_f64 / 1152.0_f64 * t19940 + 7.0_f64 / 1152.0_f64 * t19942 + t5246 * t19945 / 768.0_f64 + t12429 * t6396 / 384.0_f64 + t3803 * t19951 / 384.0_f64 - 119.0_f64 / 3456.0_f64 * t12346 - 119.0_f64 / 13824.0_f64 * t12366 + t3803 * t19958 / 768.0_f64 - t3803 * t19962 / 3072.0_f64 + t5246 * t19966 / 1536.0_f64 + t16394 * t5259 / 384.0_f64 - t3803 * t19972 / 1536.0_f64 - t3803 * t19976 / 3072.0_f64 - 5.0_f64 / 768.0_f64 * t3803 * t19981 + t3803 * t19986 / 768.0_f64 + t3803 * t19991 / 384.0_f64 + 5.0_f64 / 768.0_f64 * t1363 * t19996 - t16400 - t16233 * t20000 / 512.0_f64 - t5246 * t20004 / 384.0_f64;
    t20007
}
