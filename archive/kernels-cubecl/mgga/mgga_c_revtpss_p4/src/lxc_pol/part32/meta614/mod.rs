//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta614<F: Float>(t108278: F, t7063: F, t108277: F, t1955: F, t22307: F, t786: F, t1444: F, t6874: F, t6862: F, t22107: F, t26028: F, t22111: F) -> (F, F, F, F, F, F, F, F) {
        let (t108279, t108282, t108371, t108379, t108448, t108502, t108508, t108510) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1954::<F>(t108278, t7063, t108277, t1955, t22307, t786, t1444, t6874, t6862, t22107, t26028, t22111);
    (t108279, t108282, t108371, t108379, t108448, t108502, t108508, t108510)
}
