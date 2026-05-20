//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2819/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2819<F: Float>(t4500: F, t62808: F, t23245: F, t2815: F, t39652: F, t39673: F, t4366: F, t4504: F, t51390: F, t51403: F, t51408: F, t62684: F, t62693: F, t62697: F, t76136: F, t820: F) -> F {
    let t76255 = t62808 * t4500;
    let t76264 = -t39652 - F::cast_from(0.39029762157531132074e-2_f64) * t62684 + F::cast_from(0.78059524315062264151e-2_f64) * t51390 + F::cast_from(0.39512695097613069592e1_f64) * t4504 * t76136 * t4366 + F::cast_from(0.46263278077393568556e-2_f64) * t39673 - F::cast_from(0.29272321618148349057e-1_f64) * t76255 + F::cast_from(0.16463622957338778996e-1_f64) * t62693 + F::cast_from(0.16463622957338778996e-1_f64) * t62697 - F::cast_from(0.51220160311720645768e-1_f64) * t51403 - F::cast_from(0.91069445034239308177e-1_f64) * t51408 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t2815 * t23245;
    t76264
}
