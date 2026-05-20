//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2406;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta635<F: Float>(t2448: F, t9292: F, t11036: F, t2435: F, t10910: F, t213: F, t10994: F, t2453: F, t138: F, t2438: F, t2771: F, t2761: F, t786: F, t867: F, t2467: F, t11043: F, t10506: F, t11032: F, t789: F, t2458: F, t2444: F, t2772: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41004, t41006, t41008, t41011, t41014, t41017) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2406::<F>(t2448, t9292, t11036, t2435, t10910, t213, t10994, t2453, t138, t2438, t2771, t2761, t786, t867);
        let (t41018, t41020, t41021, t41026, t41029, t41032) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2407::<F>(t2467, t41017, t11043, t2453, t10506, t11032, t786, t789, t2458, t2761, t2444, t2772, t689);
    (t41004, t41006, t41008, t41011, t41014, t41017, t41018, t41020, t41021, t41026, t41029, t41032)
}
