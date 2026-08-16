//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1452;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1453;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta472<F: Float>(t14524: F, t51297: F, t136: F, t2457: F, t39680: F, t6022: F, t10073: F, t18746: F, t18742: F, t10069: F, t2718: F, t6041: F, t18729: F, t2470: F, t2798: F, t2482: F, t6016: F, t879: F, t14563: F, t14568: F, t10535: F, t6017: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t62874, t62907, t62909, t62920, t62922, t62929) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1452::<F>(t14524, t51297, t136, t2457, t39680, t6022, t10073, t18746, t18742, t10069, t2718, t6041);
        let (t62952, t62967, t62983, t62999) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1453::<F>(t18729, t2470, t2798, t2482, t6016, t879, t14563, t14568, t10535, t136, t2457, t6017);
    (t62874, t62907, t62909, t62920, t62922, t62929, t62952, t62967, t62983, t62999)
}
