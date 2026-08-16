//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1871;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta553<F: Float>(t1032: F, t5710: F, t1426: F, t7063: F, t1892: F, t25877: F, t1955: F, t25981: F, t5677: F, t820: F, t844: F, t241: F, t94491: F) -> (F, F, F, F, F, F, F) {
        let (t97961, t97962, t98040, t98041, t98050, t98108, t98115) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1871::<F>(t1032, t5710, t1426, t7063, t1892, t25877, t1955, t25981, t5677, t820, t844, t241, t94491);
    (t97961, t97962, t98040, t98041, t98050, t98108, t98115)
}
