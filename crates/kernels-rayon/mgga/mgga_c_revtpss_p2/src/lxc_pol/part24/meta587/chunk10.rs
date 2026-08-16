//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1834/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1834(t14193: f64, t22005: f64, t22321: f64, t46515: f64, t46518: f64, t48036: f64, t6844: f64, t74999: f64, t75005: f64, t75021: f64, t75026: f64, t75068: f64, t820: f64, t85638: f64, t86468: f64) -> f64 {
    let t92347 = -0.43902994552903410657e-1_f64 * t74999 + 0.18505311230957427423e-1_f64 * t48036 - 0.13878983423218070567e-1_f64 * t75005 + 0.78059524315062264152e-1_f64 * t75021 + 0.39029762157531132075e-2_f64 * t75026 - 0.39512695097613069592e1_f64 * t820 * t22321 * t6844 - t46515 - 0.39029762157531132076e-1_f64 * t86468 - 0.23707617058567841754e2_f64 * t14193 * t22005 * t85638 + t46518 + 0.13878983423218070567e-1_f64 * t75068;
    t92347
}
