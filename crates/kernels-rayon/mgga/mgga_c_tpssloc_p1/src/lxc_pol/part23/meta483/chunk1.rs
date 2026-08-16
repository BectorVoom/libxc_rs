//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1464/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1464(t78791: f64, t78792: f64, t78794: f64, t79006: f64, t6224: f64, t11721: f64, t1213: f64, t1214: f64, t15503: f64, t19083: f64, t22246: f64, t22271: f64, t22309: f64, t248: f64, t45030: f64, t475: f64, t488: f64, t5002: f64, t53336: f64, t6164: f64, t6169: f64, t6211: f64, t65628: f64, t65632: f64, t65647: f64, t65664: f64, t65689: f64, t72403: f64) -> (f64, f64, f64) {
    let t79008 = t78791 + t78792 + t78794 + t79006;
    let t79018 = t6224 * t6224;
    let t79024 = -t65628 / 324.0_f64 + t65632 / 2304.0_f64 + t5002 * t22246 / 768.0_f64 + t65647 / 3456.0_f64 + 19.0_f64 / 288.0_f64 * t6169 * t6164 * t488 - 19.0_f64 / 1296.0_f64 * t65664 - t15503 * t22271 / 24.0_f64 - t53336 * t22309 / 24.0_f64 + t1213 * t248 * t1214 * t79008 * t475 / 3072.0_f64 + t19083 * t6211 / 36.0_f64 + t72403 / 72.0_f64 + t65689 / 1728.0_f64 - 3.0_f64 / 256.0_f64 * t45030 * t248 * t1214 * t79018 * t11721;
    (t79008, t79018, t79024)
}
