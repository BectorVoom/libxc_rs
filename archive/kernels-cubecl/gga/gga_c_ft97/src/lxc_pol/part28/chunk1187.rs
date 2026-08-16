//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1187/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1187<F: Float>(t147: F, t35238: F, t5: F, t1080: F, t140469: F, t147091: F, t147132: F, t147159: F, t147195: F, t147224: F, t147614: F, t149270: F, t149301: F, t149335: F, t149369: F, t149404: F, t149432: F, t149458: F, t149491: F, t149607: F, t149630: F, t184: F, t21: F, t33231: F, t33234: F, t35239: F, t363: F, t3660: F, t3665: F, t3668: F, t3674: F, t3678: F, t650: F, t920: F) -> F {
    let t148 = F::cast_from(10000000.0_f64) <= t147;
    let t149639 = t5 * t35238;
    let t149661 = piecewise3::<F>(t148, F::cast_from(0.0_f64), t5 * (t147091 + t147132 + t147159 + t147195 + t147224 + t147614 + t149270 + t149301 + t149335 + t149369 + t149404 + t149432 + t149458 + t149491 + t149607 + t149630) * t184 * t21 / F::cast_from(4.0_f64) + t149639 * t650 / F::cast_from(4.0_f64) + t5 * t35239 * t363 / F::cast_from(4.0_f64) + t140469 * t1080 / F::cast_from(4.0_f64) + t33234 * t3660 / F::cast_from(4.0_f64) + t33234 * t3665 / F::cast_from(4.0_f64) + t33234 * t3668 / F::cast_from(4.0_f64) + t5 * t33231 * t920 / F::cast_from(4.0_f64) + t33234 * t3674 / F::cast_from(4.0_f64) + t33234 * t3678 / F::cast_from(2.0_f64));
    t149661
}
