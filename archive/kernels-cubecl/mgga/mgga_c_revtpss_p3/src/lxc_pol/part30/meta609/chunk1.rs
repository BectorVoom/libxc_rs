//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2076/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2076<F: Float>(t1444: F, t25921: F, t25930: F, t25931: F, t27903: F, t27960: F, t28003: F, t5774: F, t7274: F, t7295: F, t7296: F, t94405: F, t94409: F, t94411: F, t94580: F, t94584: F, t94591: F, t97719: F, t97734: F, t97737: F, t97742: F) -> F {
    let t97752 = t97719 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t28003 - F::cast_from(0.72280234901709995518e-2_f64) * t94405 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t27960 * t1444 - t94409 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t27903 + F::cast_from(0.9757440539382783019e-2_f64) * t94411 - t97734 + F::cast_from(0.13009920719177044025e-2_f64) * t94580 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t25931 * t97737 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t25931 * t97742 + F::cast_from(0.17347256376410398924e1_f64) * t7295 * t7296 * t7274 * t5774 - F::cast_from(0.54878743191129263322e-2_f64) * t94584 + F::cast_from(0.91399340044406952588e-2_f64) * t94591;
    t97752
}
