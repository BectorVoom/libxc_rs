//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2707/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2707(t1827: f64, t54532: f64, t16232: f64, t5234: f64, t12419: f64, t12429: f64, t1363: f64, t16208: f64, t16226: f64, t16235: f64, t16278: f64, t16312: f64, t19855: f64, t19871: f64, t19962: f64, t20468: f64, t3719: f64, t3734: f64, t3795: f64, t3853: f64, t3870: f64, t39978: f64, t40065: f64, t40070: f64, t40079: f64, t5235: f64, t5246: f64, t5289: f64, t5334: f64, t5344: f64, t54178: f64, t57033: f64, t57041: f64, t57044: f64, t57046: f64, t57057: f64, t57071: f64, t6330: f64, t6347: f64, t820: f64) -> f64 {
    let t57073 = t54532 * t1827;
    let t57081 = t5234 * t16232;
    let t57084 = -119.0_f64 / 3456.0_f64 * t40065 + 595.0_f64 / 3456.0_f64 * t40079 + t57033 * t3795 / 1536.0_f64 + 35.0_f64 / 128.0_f64 * t1363 * t40070 * t820 * t6330 * t3734 + 119.0_f64 / 6912.0_f64 * t57041 + 5.0_f64 / 96.0_f64 * t5334 * t57044 * t57046 * t20468 * t16312 + 5.0_f64 / 768.0_f64 * t1363 * t3870 * t820 * t6347 * t3719 + 7.0_f64 / 2304.0_f64 * t57057 - t19855 * t3853 / 3072.0_f64 - t54178 * t1827 / 1536.0_f64 - t16278 * t5289 / 768.0_f64 - t5235 * t16208 / 1536.0_f64 - 5.0_f64 / 192.0_f64 * t5344 * t57044 * t57046 * t16226 - 7.0_f64 / 1152.0_f64 * t57071 - 119.0_f64 / 6912.0_f64 * t57073 + 5.0_f64 / 384.0_f64 * t5246 * t12419 * t19871 * t39978 - t12429 * t19962 / 1536.0_f64 - t57081 * t16235 / 256.0_f64;
    t57084
}
