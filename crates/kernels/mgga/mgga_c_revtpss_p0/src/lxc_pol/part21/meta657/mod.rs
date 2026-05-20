//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2447;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta657<F: Float>(t3105: F, t3223: F, t11960: F, t351: F, t361: F, t369: F, t1041: F, t11262: F, t3135: F, t1033: F, t1036: F, t1038: F, t1063: F, t11160: F, t247: F, t3109: F, t11620: F, t73: F, t12166: F, t15905: F, t994: F, t11662: F, t11710: F, t4892: F) -> (F, F, F, F, F, F, F, F) {
        let (t42571, t42576, t42580, t42584) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2447::<F>(t3105, t3223, t11960, t351, t361, t369, t1041, t11262, t3135, t1033, t1036, t1038);
        let (t42606, t42610, t42621, t42637) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2448::<F>(t1063, t11160, t247, t3109, t11620, t73, t12166, t15905, t994, t11662, t11710, t4892);
    (t42571, t42576, t42580, t42584, t42606, t42610, t42621, t42637)
}
