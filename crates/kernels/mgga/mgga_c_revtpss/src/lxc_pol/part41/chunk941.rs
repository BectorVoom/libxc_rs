//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 941/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk941<F: Float>(t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F, t701: F, t682: F) -> (F,) {
    let t9305 = -0.25319e1 * t9283 + 0.16879333333333333333e1 * t9286 - 0.19692555555555555555e1 * t9289 - 0.93011851851851851854e0 * t9292 + 0.13651666666666666667e0 * t9296 - 0.27303333333333333333e0 * t9298 - 0.3185388888888888889e0 * t9300 - 0.36514074074074074075e0 * t9303;
    let t9306 = t9305 * t701;
    let t9308 = 1.0 * t682 * t9306;
    (t9308,)
}
