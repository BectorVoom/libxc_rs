//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3235/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3235<F: Float>(t1469: F, t1486: F, t72: F, t1494: F, t18281: F, t1927: F, t21686: F, t21687: F, t21727: F, t22662: F, t22672: F, t22739: F, t36: F, t4186: F, t4196: F, t5825: F, t5869: F, t608: F, t60823: F, t627: F, t6977: F, t70: F, t76397: F, t7719: F, t78770: F, t85: F) -> F {
    let t85161 = t1469 * t1486 * t72;
    let t85177 = -t21727 * t1494 / F::cast_from(4.0_f64) - t4196 * t5869 / F::cast_from(4.0_f64) - t608 * t22739 / F::cast_from(12.0_f64) - t4186 * t70 * t72 * t22662 / F::cast_from(4.0_f64) - t60823 * t22662 / F::cast_from(4.0_f64) - t21686 * t6977 * t5825 / F::cast_from(4.0_f64) - t21686 * t1927 * t18281 / F::cast_from(4.0_f64) - t85161 * t21687 / F::cast_from(2.0_f64) - t21686 * t7719 * t4186 / F::cast_from(2.0_f64) - t78770 * t70 * t85 / F::cast_from(12.0_f64) - t36 * t76397 * t70 * t85 / F::cast_from(12.0_f64) - t22672 * t627 * t85 / F::cast_from(12.0_f64);
    t85177
}
