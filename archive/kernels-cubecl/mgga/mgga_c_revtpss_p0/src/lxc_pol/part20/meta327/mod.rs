//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1242;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta327<F: Float>(t13037: F, t474: F, t11243: F, t479: F, t13036: F, t1248: F, t3601: F, t482: F, t3603: F, t471: F) -> (F, F, F, F, F, F, F, F) {
        let (t13038, t13039, t13040, t13041, t13042, t13043) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1242::<F>(t13037, t474, t11243, t479, t13036, t1248, t3601);
        let (t13044, t13045) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1243::<F>(t13043, t482, t3603, t471);
    (t13038, t13039, t13040, t13041, t13042, t13043, t13044, t13045)
}
