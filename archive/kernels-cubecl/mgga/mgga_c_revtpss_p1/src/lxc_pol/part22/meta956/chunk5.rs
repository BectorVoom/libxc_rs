//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3205/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3205<F: Float>(t13335: F, t13343: F, t13346: F, t13389: F, t1487: F, t1494: F, t21690: F, t21769: F, t21805: F, t2291: F, t2292: F, t2312: F, t4218: F, t4238: F, t5819: F, t5820: F, t5855: F, t5869: F, t60717: F, t60778: F, t628: F, t641: F, t70: F, t71: F, t77: F, t85: F) -> F {
    let t60793 = -t60717 * t70 * t85 / F::cast_from(6.0_f64) + t21769 * t641 / F::cast_from(12.0_f64) + t5855 * t2312 / F::cast_from(24.0_f64) + t13335 * t1494 / F::cast_from(12.0_f64) + t4218 * t4238 / F::cast_from(6.0_f64) + t1487 * t13389 / F::cast_from(12.0_f64) + t2292 * t5869 / F::cast_from(24.0_f64) + t628 * t21805 / F::cast_from(12.0_f64) + t71 * t77 * t60778 / F::cast_from(24.0_f64) - t5819 * t2291 * t85 / F::cast_from(12.0_f64) - t21690 * t641 / F::cast_from(6.0_f64) - t5820 * t2312 / F::cast_from(12.0_f64) - t13343 * t1494 / F::cast_from(6.0_f64) - t13346 * t1494 / F::cast_from(3.0_f64);
    t60793
}
