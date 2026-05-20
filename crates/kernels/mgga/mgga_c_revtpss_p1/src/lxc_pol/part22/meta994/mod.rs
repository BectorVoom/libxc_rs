//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta994 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta994<F: Float>(t19013: F, t698: F, t19016: F, t2439: F, t6138: F, t18960: F, t18963: F, t18966: F, t141: F, t2908: F, t63353: F, t11341: F, t63302: F) -> (F, F, F, F, F, F, F, F) {
        let (t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3381::<F>(t19013, t698, t19016, t2439, t6138, t18960, t18963, t18966, t141, t2908, t63353, t11341, t63302);
    (t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557)
}
