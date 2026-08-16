//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1460/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1460(t11310: f64, t11365: f64, t11420: f64, t15126: f64, t15146: f64, t15207: f64, t1682: f64, t1694: f64, t18622: f64, t21839: f64, t21842: f64, t21845: f64, t21887: f64, t21939: f64, t3332: f64, t3376: f64, t3401: f64, t6052: f64, t6056: f64, t6069: f64, t6084: f64, t6088: f64, t71672: f64, t78225: f64, t78327: f64, t78329: f64, t78331: f64, t78333: f64, t78335: f64, t78355: f64) -> f64 {
    let t78944 = -t78327 - t78329 - t78331 - t78333 - t78335 + 0.20779030926817756511e3_f64 * t15126 * t21839 - 0.62337092780453269531e3_f64 * t11365 * t6088 * t6084 - 0.46785788981077169656e1_f64 * t3376 * t21939 * t1694 + 0.69263436422725855036e2_f64 * t3401 * t71672 * t1694 + 0.61524113149298439947e4_f64 * t11310 * t18622 * t6084 + 0.21053605041484726346e2_f64 * t3401 * t6069 * t6084 - 24.0_f64 * t15207 * t21842 + 0.3859675079686208416e3_f64 * t15146 * t21845 - 0.11579025239058625248e4_f64 * t11420 * t6056 * t6052 - 8.0_f64 * t3332 * t21887 * t1682 - 0.19751673498613801407e-1_f64 * t78225 - t78355;
    t78944
}
