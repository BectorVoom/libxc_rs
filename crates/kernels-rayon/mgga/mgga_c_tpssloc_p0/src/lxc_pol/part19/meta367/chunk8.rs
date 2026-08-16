//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1352/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1352(t1025: f64, t10403: f64, t10426: f64, t10428: f64, t10480: f64, t10501: f64, t10517: f64, t10915: f64, t10949: f64, t10965: f64, t13980: f64, t13985: f64, t14213: f64, t3071: f64, t3098: f64, t3117: f64, t3123: f64, t3130: f64, t39110: f64, t42639: f64, t43103: f64, t43110: f64, t43114: f64, t43118: f64, t43120: f64, t4582: f64, t4594: f64, t973: f64, t974: f64, t998: f64) -> f64 {
    let t43141 = -t3117 * t10915 / 192.0_f64 + 7.0_f64 / 486.0_f64 * t43103 + t973 * t974 * t998 * t39110 / 288.0_f64 + t43110 / 108.0_f64 + 19.0_f64 / 288.0_f64 * t10517 * t3123 - t43114 / 1728.0_f64 + t43118 / 1152.0_f64 - t43120 * t1025 / 48.0_f64 + t10949 * t10428 / 128.0_f64 + t3130 * t4582 * t42639 * t4594 / 384.0_f64 + 3.0_f64 / 256.0_f64 * t10480 * t4582 * t10426 * t13985 - t10965 * t3098 / 384.0_f64 - 5.0_f64 / 576.0_f64 * t3117 * t10501 + t10403 * t3071 * t13980 * t14213 / 192.0_f64;
    t43141
}
