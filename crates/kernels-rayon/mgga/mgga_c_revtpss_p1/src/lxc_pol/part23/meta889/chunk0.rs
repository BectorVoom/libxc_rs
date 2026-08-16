//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2819/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2819(t4500: f64, t62808: f64, t23245: f64, t2815: f64, t39652: f64, t39673: f64, t4366: f64, t4504: f64, t51390: f64, t51403: f64, t51408: f64, t62684: f64, t62693: f64, t62697: f64, t76136: f64, t820: f64) -> f64 {
    let t76255 = t62808 * t4500;
    let t76264 = -t39652 - 0.39029762157531132074e-2_f64 * t62684 + 0.78059524315062264151e-2_f64 * t51390 + 0.39512695097613069592e1_f64 * t4504 * t76136 * t4366 + 0.46263278077393568556e-2_f64 * t39673 - 0.29272321618148349057e-1_f64 * t76255 + 0.16463622957338778996e-1_f64 * t62693 + 0.16463622957338778996e-1_f64 * t62697 - 0.51220160311720645768e-1_f64 * t51403 - 0.91069445034239308177e-1_f64 * t51408 - 0.65854491829355115987e0_f64 * t820 * t2815 * t23245;
    t76264
}
