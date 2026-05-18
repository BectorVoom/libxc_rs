//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1064/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1064<F: Float>(t34491: F, t376: F, t89: F, t34503: F, t22873: F, t28: F, t6454: F, t25846: F, t5507: F, t3103: F, t32338: F, t137215: F, t137229: F, t137652: F, t137654: F, t137657: F, t137659: F, t145667: F, t145669: F, t145673: F, t145676: F, t145681: F) -> (F, F, F, F, F, F) {
    let t145684 = t89 * t376 * t34491;
    let t145687 = t89 * t376 * t34503;
    let t145691 = t89 * t28 * t22873 * t6454;
    let t145695 = t89 * t28 * t5507 * t25846;
    let t145699 = t89 * t28 * t32338 * t3103;
    let t145701 = t145667 + t137652 + t137654 - t137657 + F::new(4.0) / F::new(3.0) * t145669 + F::new(2.0) / F::new(9.0) * t145673 - F::new(8.0) / F::new(3.0) * t145676 + t137215 / F::new(3.0) - t137659 - t137229 / F::new(9.0) - F::new(4.0) / F::new(3.0) * t145681 + F::new(2.0) * t145684 - F::new(2.0) / F::new(3.0) * t145687 + F::new(4.0) * t145691 + F::new(4.0) * t145695 - F::new(6.0) * t145699;
    (t145684, t145687, t145691, t145695, t145699, t145701)
}
