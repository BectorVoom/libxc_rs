//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3197/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3197<F: Float>(t21169: F, t5373: F, t21251: F, t1222: F, t17475: F, t5308: F, t5312: F, t59041: F, t71320: F, t71329: F, t71341: F, t81160: F, t81165: F, t81169: F, t81190: F, t81207: F) -> F {
    let t83992 = t5373 * t21169;
    let t83994 = t5373 * t21251;
    let t83996 = -t1222 * t5308 * t81207 / F::cast_from(144.0_f64) - t1222 * t5308 * t81190 / F::cast_from(16.0_f64) + t71320 / F::cast_from(27.0_f64) - F::cast_from(0.11433071498151929859e-2_f64) * t71329 - F::cast_from(7.0_f64) / F::cast_from(216.0_f64) * t1222 * t17475 * t81160 - F::cast_from(7.0_f64) / F::cast_from(54.0_f64) * t1222 * t17475 * t81165 - F::cast_from(0.11433071498151929859e-2_f64) * t71341 + t1222 * t5312 * t81169 / F::cast_from(12.0_f64) - t59041 - t83992 / F::cast_from(81.0_f64) + t83994 / F::cast_from(108.0_f64);
    t83996
}
