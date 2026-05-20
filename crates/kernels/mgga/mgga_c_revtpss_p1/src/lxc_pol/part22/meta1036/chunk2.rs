//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3625/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3625<F: Float>(t1284: F, t21333: F, t68243: F, t68245: F, t68247: F, t68250: F, t68602: F, t68604: F, t68608: F, t68611: F, t68613: F, t68621: F, t68625: F, t68628: F) -> (F, F) {
    let t68674 = t21333 * t1284;
    let t68679 = -t68243 - t68245 - t68247 - t68250 - t68602 - t68604 - t68608 - t68611 - t68613 + t68621 + t68625 + t68628;
    (t68674, t68679)
}
