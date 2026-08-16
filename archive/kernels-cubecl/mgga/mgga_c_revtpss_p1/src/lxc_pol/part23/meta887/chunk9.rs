//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2810/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2810<F: Float>(t23383: F, t2465: F, t686: F, t72: F, t10995: F, t23403: F, t1579: F, t18324: F, t18784: F, t2770: F, t41060: F, t4474: F, t51211: F, t51213: F, t51217: F, t51234: F, t51237: F, t51240: F, t51246: F, t51260: F, t51263: F, t62549: F, t62572: F, t865: F) -> F {
    let t76058 = t2465 * t23383 * t72 * t686;
    let t76062 = t10995 * t23403 * t72 * t686;
    let t76077 = -F::cast_from(0.9757440539382783019e-2_f64) * t76058 + F::cast_from(0.58544643236296698112e-1_f64) * t76062 + F::cast_from(0.91069445034239308177e-1_f64) * t51211 + F::cast_from(0.51220160311720645768e-1_f64) * t51213 + t51217 + F::cast_from(0.30356481678079769392e-1_f64) * t41060 + F::cast_from(0.39512695097613069592e1_f64) * t4474 * t18324 + t51234 - F::cast_from(0.16463622957338778996e-1_f64) * t62549 - F::cast_from(0.78059524315062264151e-2_f64) * t51237 + t51240 + F::cast_from(0.58911598146606471821e-3_f64) * t51246 - F::cast_from(0.32927245914677557992e-1_f64) * t62572 + F::cast_from(0.39512695097613069591e1_f64) * t865 * t2770 * t1579 * t18784 - t51260 + t51263;
    t76077
}
