//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 682/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk682<F: Float>(t12362: F, t12571: F, t16679: F, t16745: F, t16748: F, t16751: F, t20536: F, t20540: F, t20551: F, t20666: F, t20669: F, t20779: F, t9166: F, t16925: F, t16928: F, t20543: F, t20547: F, t20554: F, t20558: F, t20562: F, t20566: F, t20570: F, t20658: F, t20663: F, t20784: F, t20839: F) -> (F, F) {
    let t20961 = -2.0 / 3.0 * t16679 + 6.0 * t20666 - t20669 / 3.0 - 4.0 / 9.0 * t12362 - t9166 - 4.0 / 3.0 * t12571 - 10.0 / 27.0 * t20536 - 2.0 * t20540 + 4.0 / 3.0 * t20551 - 3.0 / 4.0 * t20779 + t16745 / 3.0 - 2.0 / 3.0 * t16748 + 2.0 / 9.0 * t16751;
    let t20971 = 3.0 / 8.0 * t20784 + t20839 / 2.0 + t16925 - 2.0 * t16928 - t20658 - 6.0 * t20663 - 2.0 / 3.0 * t20554 + t20558 + t20562 - 2.0 * t20566 - 2.0 * t20570 + 2.0 * t20543 + 2.0 / 3.0 * t20547;
    (t20961, t20971)
}
