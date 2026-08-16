//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2810/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2810(t23383: f64, t2465: f64, t686: f64, t72: f64, t10995: f64, t23403: f64, t1579: f64, t18324: f64, t18784: f64, t2770: f64, t41060: f64, t4474: f64, t51211: f64, t51213: f64, t51217: f64, t51234: f64, t51237: f64, t51240: f64, t51246: f64, t51260: f64, t51263: f64, t62549: f64, t62572: f64, t865: f64) -> f64 {
    let t76058 = t2465 * t23383 * t72 * t686;
    let t76062 = t10995 * t23403 * t72 * t686;
    let t76077 = -0.9757440539382783019e-2_f64 * t76058 + 0.58544643236296698112e-1_f64 * t76062 + 0.91069445034239308177e-1_f64 * t51211 + 0.51220160311720645768e-1_f64 * t51213 + t51217 + 0.30356481678079769392e-1_f64 * t41060 + 0.39512695097613069592e1_f64 * t4474 * t18324 + t51234 - 0.16463622957338778996e-1_f64 * t62549 - 0.78059524315062264151e-2_f64 * t51237 + t51240 + 0.58911598146606471821e-3_f64 * t51246 - 0.32927245914677557992e-1_f64 * t62572 + 0.39512695097613069591e1_f64 * t865 * t2770 * t1579 * t18784 - t51260 + t51263;
    t76077
}
