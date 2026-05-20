//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta574<F: Float>(t1358: F, t2439: F, t785: F, t8085: F, t1364: F, t28905: F, t786: F, t96187: F, t97688: F, t28791: F, t689: F, t25899: F, t136: F, t2457: F, t8094: F, t94589: F, t26072: F, t28845: F, t28840: F, t686: F, t72: F, t25895: F, t2470: F, t28779: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t102139, t102143, t102164, t102165, t102167) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1899::<F>(t1358, t2439, t785, t8085, t1364, t28905, t786, t96187, t97688, t28791, t689, t25899);
        let (t102204, t102205, t102213, t102215, t102217, t102218) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1900::<F>(t136, t2457, t8094, t94589, t26072, t28845, t28840, t686, t72, t25895, t2470, t28779);
    (t102139, t102143, t102164, t102165, t102167, t102204, t102205, t102213, t102215, t102217, t102218)
}
