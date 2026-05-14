//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 928/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk928<F: Float>(t25846: F, t28: F, t5507: F, t89: F, t3103: F, t32338: F, t137215: F, t137229: F, t137652: F, t137654: F, t137657: F, t137659: F, t145667: F, t145669: F, t145673: F, t145676: F, t145681: F, t145684: F, t145687: F, t145691: F) -> (F, F, F) {
    let t145695 = t89 * t28 * t5507 * t25846;
    let t145699 = t89 * t28 * t32338 * t3103;
    let t145701 = t145667 + t137652 + t137654 - t137657 + 4.0 / 3.0 * t145669 + 2.0 / 9.0 * t145673 - 8.0 / 3.0 * t145676 + t137215 / 3.0 - t137659 - t137229 / 9.0 - 4.0 / 3.0 * t145681 + 2.0 * t145684 - 2.0 / 3.0 * t145687 + 4.0 * t145691 + 4.0 * t145695 - 6.0 * t145699;
    (t145695, t145699, t145701)
}
