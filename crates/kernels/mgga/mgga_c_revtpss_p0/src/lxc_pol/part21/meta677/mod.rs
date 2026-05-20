//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta677<F: Float>(t43813: F, t43816: F, t3431: F, t408: F, t3434: F, t1126: F, t12247: F, t3800: F, t12773: F, t12784: F, t12772: F, t12835: F, t3625: F) -> (F, F, F, F, F, F, F, F) {
        let (t44039, t44040, t44091, t44093, t44101, t44126, t44200, t44215) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2488::<F>(t43813, t43816, t3431, t408, t3434, t1126, t12247, t3800, t12773, t12784, t12772, t12835, t3625);
    (t44039, t44040, t44091, t44093, t44101, t44126, t44200, t44215)
}
