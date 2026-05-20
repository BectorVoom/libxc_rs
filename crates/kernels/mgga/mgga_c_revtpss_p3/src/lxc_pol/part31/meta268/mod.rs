//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1200;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1201;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta268<F: Float>(t1444: F, t2022: F, t7296: F, t1385: F, t1426: F, t1398: F, t543: F, t545: F, t7274: F, t2028: F, t1445: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7275: F, t7279: F, t7288: F, t7291: F, t7292: F, t7295: F, t532: F, t1450: F, t2014: F, t1448: F, t4147: F, t2034: F, t118: F, t1310: F, t1453: F, t1932: F, t2007: F, t2011: F, t508: F, t569: F, t649: F, t651: F, t671: F, t6983: F, t6985: F, t6990: F, t6992: F, t6995: F, t7005: F, t7007: F, t7221: F, t7231: F, t7236: F, t7241: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7298, t7301) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1200::<F>(t1444, t2022, t7296, t1385, t1426);
        let (t7303, t7304, t7307, t7308, t7311) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1201::<F>(t1398, t2022, t543, t7301, t545, t7274, t2028, t1445, t2027, t2030, t213, t561, t7245, t7248, t7275, t7279, t7288, t7291, t7292, t7295, t7298);
        let (t7312, t7313, t7315, t7316, t7318) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1202::<F>(t532, t7311, t1450, t2014, t1448, t4147, t2034, t118, t1310, t1453, t1932, t2007, t2011, t508, t569, t649, t651, t671, t6983, t6985, t6990, t6992, t6995, t7005, t7007, t7221, t7231, t7236, t7241);
    (t7298, t7301, t7303, t7304, t7307, t7308, t7311, t7312, t7313, t7315, t7316, t7318)
}
