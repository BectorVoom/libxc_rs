//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3474/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3474<F: Float>(t64335: F, t64338: F, t64340: F, t64342: F, t64344: F, t64346: F, t64404: F, t64512: F, t64521: F, t64523: F, t64527: F, t64529: F, t64531: F) -> F {
    let t65398 = -t64512 + t64335 + t64338 + t64340 + t64342 + t64344 - t64346 - t64521 - t64404 + t64523 + t64527 + t64529 - t64531;
    t65398
}
