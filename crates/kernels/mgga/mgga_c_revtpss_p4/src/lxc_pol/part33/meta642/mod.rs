//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta642<F: Float>(t1479: F, t2282: F, t1204: F, t8190: F, t1276: F, t42859: F, t13038: F, t2149: F, t1203: F, t471: F, t355: F, t5352: F) -> (F, F, F, F, F, F) {
        let (t104379, t104465, t104480, t104482, t104505, t104510) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2091::<F>(t1479, t2282, t1204, t8190, t1276, t42859, t13038, t2149, t1203, t471, t355, t5352);
    (t104379, t104465, t104480, t104482, t104505, t104510)
}
