//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2574;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta730<F: Float>(t221: F, t4018: F, t4019: F, t9891: F, t1389: F, t3964: F, t40604: F, t3961: F, t9741: F, t10111: F, t22: F, t4092: F, t39515: F, t4083: F, t10043: F, t9303: F, t10014: F, t10019: F, t268: F, t4101: F, t543: F, t675: F, t9890: F, t10139: F, t281: F, t4056: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47333, t47337, t47338, t47348) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2574::<F>(t221, t4018, t4019, t9891, t1389, t3964, t40604, t3961, t9741, t10111, t22, t4092);
        let (t47351, t47352, t47354, t47359, t47364) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2575::<F>(t39515, t4083, t10043, t9303, t10014, t10019, t268, t4101, t543, t675, t9890, t10139, t281, t4056, t68);
    (t47333, t47337, t47338, t47348, t47351, t47352, t47354, t47359, t47364)
}
