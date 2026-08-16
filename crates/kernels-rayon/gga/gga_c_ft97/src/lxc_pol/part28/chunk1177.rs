//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1177/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1177(t1058: f64, t7312: f64, t104364: f64, t1349: f64, t147866: f64, t147887: f64, t23925: f64, t26533: f64, t26538: f64, t26567: f64, t26569: f64, t26581: f64, t26805: f64, t28: f64, t32714: f64, t32967: f64, t34800: f64, t34803: f64, t379: f64, t5766: f64, t5772: f64, t5778: f64, t7346: f64, t9073: f64) -> f64 {
    let t149309 = t7312 * t1058;
    let t149335 = -t32714 * t26805 / 18.0_f64 - t32714 * t26569 / 18.0_f64 + t5772 * t9073 * t149309 * t379 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t5778 * t104364 + t1349 * t28 * t32967 * t26538 - 2.0_f64 / 3.0_f64 * t5766 * t34803 - 2.0_f64 / 3.0_f64 * t1349 * t28 * t23925 * t26567 + t1349 * t28 * t32967 * t26533 + t5766 * t34800 + t26581 * t7346 / 3.0_f64 + 8.0_f64 * t147887 - 12.0_f64 * t147866;
    t149335
}
