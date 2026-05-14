//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1256/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1256<F: Float>(t104336: F, t104599: F, t104619: F, t1349: F, t1389: F, t16661: F, t17066: F, t17076: F, t1969: F, t23408: F, t23413: F, t23925: F, t26553: F, t26569: F, t26817: F, t28: F, t30117: F, t30119: F, t30127: F, t30137: F, t39653: F, t40830: F, t4462: F, t5766: F, t5772: F, t5773: F, t609: F, t6580: F, t925: F, t9432: F, t94984: F) -> (F,) {
    let t119440 = 48.0 * t39653 * t30127 * t609 - 2.0 / 3.0 * t6580 * t26553 - t5772 * t1969 * t23408 * t4462 / 18.0 - t16661 * t1389 - 2.0 / 81.0 * t94984 - t5772 * t1969 * t104336 * t925 / 9.0 + t104599 - t26817 * t26569 / 9.0 - 2.0 / 3.0 * t5766 * t30119 - 2.0 / 3.0 * t1349 * t28 * t23925 * t30117 - 4.0 * t5772 * t40830 * t5773 * t17066 + 2.0 * t5772 * t9432 * t5773 * t17076 - t23413 * t30137 / 9.0 + 4.0 / 27.0 * t104619;
    (t119440,)
}
