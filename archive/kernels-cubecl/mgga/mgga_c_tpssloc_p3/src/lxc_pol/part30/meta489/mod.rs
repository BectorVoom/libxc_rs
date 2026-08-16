//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1794;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta489<F: Float>(t25261: F, t829: F, t1510: F, t22992: F, t13380: F, t232: F, t6646: F, t1888: F, t1499: F, t23002: F, t23014: F, t23026: F, t23028: F, t23032: F, t23166: F, t23169: F, t23174: F, t25239: F, t25243: F, t25246: F, t25252: F, t25256: F, t25259: F, t2617: F, t4291: F, t6660: F, t7533: F, t812: F) -> (F, F, F, F, F) {
        let (t25262, t25269, t25272, t25273, t25276) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1794::<F>(t25261, t829, t1510, t22992, t13380, t232, t6646, t1888, t1499, t23002, t23014, t23026, t23028, t23032, t23166, t23169, t23174, t25239, t25243, t25246, t25252, t25256, t25259, t2617, t4291, t6660, t7533, t812);
    (t25262, t25269, t25272, t25273, t25276)
}
