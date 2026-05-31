//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1250/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1250<F: Float>(t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F, t701: F, t682: F) -> F {
    let t9305 = -F::cast_from(0.25319e1_f64) * t9283 + F::cast_from(0.16879333333333333333e1_f64) * t9286 - F::cast_from(0.19692555555555555555e1_f64) * t9289 - F::cast_from(0.93011851851851851854e0_f64) * t9292 + F::cast_from(0.13651666666666666667e0_f64) * t9296 - F::cast_from(0.27303333333333333333e0_f64) * t9298 - F::cast_from(0.3185388888888888889e0_f64) * t9300 - F::cast_from(0.36514074074074074075e0_f64) * t9303;
    let t9306 = t9305 * t701;
    let t9308 = F::cast_from(1.0_f64) * t682 * t9306;
    t9308
}
