//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta406<F: Float>(t11249: F, t13045: F, t13044: F, t1042: F, t13040: F, t3597: F, t13036: F, t3603: F, t13032: F, t3609: F, t1244: F, t471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13046, t13047, t13048, t13051, t13052, t13053, t13054, t13055, t13058, t13061, t13062, t13063) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1870::<F>(t11249, t13045, t13044, t1042, t13040, t3597, t13036, t3603, t13032, t3609, t1244, t471);
    (t13046, t13047, t13048, t13051, t13052, t13053, t13054, t13055, t13058, t13061, t13062, t13063)
}
