//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1329/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1329(t42661: f64, t42679: f64, t42693: f64, t42713: f64, t10510: f64, t3114: f64, t1020: f64, t1021: f64, t1023: f64, t1025: f64, t1041: f64, t10426: f64, t10433: f64, t1046: f64, t10463: f64, t10863: f64, t10876: f64, t10952: f64, t14164: f64, t248: f64, t3039: f64, t3048: f64, t3057: f64, t3132: f64, t360: f64, t39097: f64, t42468: f64, t42622: f64, t42624: f64, t42639: f64, t42648: f64, t42651: f64, t42653: f64, t42658: f64, t4582: f64, t973: f64, t974: f64) -> (f64, f64) {
    let t42715 = t42661 + t42679 + t42693 + t42713;
    let t42721 = t3114 * t10510;
    let t42723 = -4.0_f64 / 81.0_f64 * t42622 - 7.0_f64 / 54.0_f64 * t973 * t974 * t42624 * t39097 - t10863 * t3057 / 72.0_f64 - t3048 * t10463 / 216.0_f64 - 3.0_f64 / 256.0_f64 * t10876 * t4582 * t10426 * t3132 - t10952 * t10433 / 256.0_f64 - t3039 * t4582 * t42639 * t1023 / 768.0_f64 + t1041 * t4582 * t14164 * t42468 / 128.0_f64 + 19.0_f64 / 216.0_f64 * t42648 * t1046 - t42651 / 54.0_f64 + 19.0_f64 / 144.0_f64 * t42653 * t1025 - 209.0_f64 / 648.0_f64 * t42658 * t1025 + t1020 * t248 * t1021 * t42715 * t360 / 3072.0_f64 - t42721 / 1152.0_f64;
    (t42715, t42723)
}
