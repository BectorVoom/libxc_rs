//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1177/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1177<F: Float>(t1058: F, t7312: F, t104364: F, t1349: F, t147866: F, t147887: F, t23925: F, t26533: F, t26538: F, t26567: F, t26569: F, t26581: F, t26805: F, t28: F, t32714: F, t32967: F, t34800: F, t34803: F, t379: F, t5766: F, t5772: F, t5778: F, t7346: F, t9073: F) -> F {
    let t149309 = t7312 * t1058;
    let t149335 = -t32714 * t26805 / F::cast_from(18.0_f64) - t32714 * t26569 / F::cast_from(18.0_f64) + t5772 * t9073 * t149309 * t379 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t28 * t5778 * t104364 + t1349 * t28 * t32967 * t26538 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5766 * t34803 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t28 * t23925 * t26567 + t1349 * t28 * t32967 * t26533 + t5766 * t34800 + t26581 * t7346 / F::cast_from(3.0_f64) + F::cast_from(8.0_f64) * t147887 - F::cast_from(12.0_f64) * t147866;
    t149335
}
