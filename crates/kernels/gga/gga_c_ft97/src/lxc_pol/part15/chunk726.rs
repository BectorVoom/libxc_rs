//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 726/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk726<F: Float>(t20660: F, t9016: F, t27: F, t89: F, t3342: F, t4714: F, t28: F, t20044: F, t519: F, t356: F, t12362: F, t16679: F, t16745: F, t16748: F, t16751: F, t16925: F, t16928: F, t20658: F, t9072: F) -> (F, F, F, F, F, F, F) {
    let t20661 = t9016 * t20660;
    let t20663 = t89 * t27 * t20661;
    let t20664 = t3342 * t4714;
    let t20666 = t89 * t28 * t20664;
    let t20667 = t519 * t20044;
    let t20669 = t89 * t356 * t20667;
    let t20676 = -F::new(2.0) / F::new(27.0) * t12362 - t16679 / F::new(9.0) - t20658 / F::new(6.0) - t20663 + t20666 - t20669 / F::new(18.0) + t16745 / F::new(18.0) - t16748 / F::new(9.0) + t16751 / F::new(27.0) - t9072 + t16925 / F::new(6.0) - t16928 / F::new(3.0);
    (t20661, t20663, t20664, t20666, t20667, t20669, t20676)
}
