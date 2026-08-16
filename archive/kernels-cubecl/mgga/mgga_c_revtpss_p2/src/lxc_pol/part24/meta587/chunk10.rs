//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1834/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1834<F: Float>(t14193: F, t22005: F, t22321: F, t46515: F, t46518: F, t48036: F, t6844: F, t74999: F, t75005: F, t75021: F, t75026: F, t75068: F, t820: F, t85638: F, t86468: F) -> F {
    let t92347 = -F::cast_from(0.43902994552903410657e-1_f64) * t74999 + F::cast_from(0.18505311230957427423e-1_f64) * t48036 - F::cast_from(0.13878983423218070567e-1_f64) * t75005 + F::cast_from(0.78059524315062264152e-1_f64) * t75021 + F::cast_from(0.39029762157531132075e-2_f64) * t75026 - F::cast_from(0.39512695097613069592e1_f64) * t820 * t22321 * t6844 - t46515 - F::cast_from(0.39029762157531132076e-1_f64) * t86468 - F::cast_from(0.23707617058567841754e2_f64) * t14193 * t22005 * t85638 + t46518 + F::cast_from(0.13878983423218070567e-1_f64) * t75068;
    t92347
}
