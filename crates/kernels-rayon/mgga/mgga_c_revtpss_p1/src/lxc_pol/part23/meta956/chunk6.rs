//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3197/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3197(t21169: f64, t5373: f64, t21251: f64, t1222: f64, t17475: f64, t5308: f64, t5312: f64, t59041: f64, t71320: f64, t71329: f64, t71341: f64, t81160: f64, t81165: f64, t81169: f64, t81190: f64, t81207: f64) -> f64 {
    let t83992 = t5373 * t21169;
    let t83994 = t5373 * t21251;
    let t83996 = -t1222 * t5308 * t81207 / 144.0_f64 - t1222 * t5308 * t81190 / 16.0_f64 + t71320 / 27.0_f64 - 0.11433071498151929859e-2_f64 * t71329 - 7.0_f64 / 216.0_f64 * t1222 * t17475 * t81160 - 7.0_f64 / 54.0_f64 * t1222 * t17475 * t81165 - 0.11433071498151929859e-2_f64 * t71341 + t1222 * t5312 * t81169 / 12.0_f64 - t59041 - t83992 / 81.0_f64 + t83994 / 108.0_f64;
    t83996
}
