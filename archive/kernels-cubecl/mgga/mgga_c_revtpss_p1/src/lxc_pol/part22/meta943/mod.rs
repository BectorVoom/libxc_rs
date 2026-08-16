//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta943 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3178;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta943<F: Float>(t12226: F, t1719: F, t12470: F, t1744: F, t12555: F, t5180: F, t12486: F, t300: F, t12553: F, t3521: F, t1261: F, t1715: F, t247: F, t44701: F, t12809: F, t12916: F, t17380: F, t3568: F, t3603: F, t1247: F, t1796: F, t42994: F, t17231: F, t3172: F, t1250: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58473, t58592, t58647, t58665, t58672, t58708, t58777) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3178::<F>(t12226, t1719, t12470, t1744, t12555, t5180, t12486, t300, t12553, t3521, t1261, t1715, t247, t44701);
        let (t58791, t58803, t58824, t58827, t58831) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3179::<F>(t12809, t12916, t17380, t3568, t3603, t1247, t1796, t42994, t1261, t17231, t3172, t1250);
    (t58473, t58592, t58647, t58665, t58672, t58708, t58777, t58791, t58803, t58824, t58827, t58831)
}
