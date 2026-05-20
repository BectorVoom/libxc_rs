//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1204;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1205;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1206;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1207;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1208;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta269<F: Float>(t1248: F, t1287: F, t7660: F, t2150: F, t473: F, t7627: F, t1204: F, t1215: F, t1295: F, t2144: F, t2149: F, t2152: F, t460: F, t7602: F, t7629: F, t7632: F, t7636: F, t7639: F, t7643: F, t7645: F, t7648: F, t7651: F, t7654: F, t7659: F, t2155: F, t3801: F, t33: F, t265: F, t502: F, t1298: F, t1300: F, t198: F, t336: F, t5023: F, t7193: F, t2159: F, t57: F, t606: F, t7214: F, dens_threshold: F, rho1: F, zeta_threshold: F, t7599: F, t670: F, t7226: F, t7228: F, t7230: F, t7584: F, t7586: F, t118: F, t1310: F, t1453: F, t2127: F, t2163: F, t2165: F, t508: F, t569: F, t649: F, t651: F, t671: F, t6990: F, t6992: F, t6995: F, t7005: F, t7236: F, t7241: F, t7314: F, t7317: F, t7591: F) -> (F, F, F, F, F, F, F, F) {
        let (t7662, t7666, t7669) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1204::<F>(t1248, t1287, t7660, t2150, t473, t7627, t1204, t1215, t1295, t2144, t2149, t2152, t460, t7602, t7629, t7632, t7636, t7639, t7643, t7645, t7648, t7651, t7654, t7659);
        let t7673 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1205::<F>(t2155, t3801);
        let (t7677, t7682) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1206::<F>(t33, t265, t502, t1298, t1300, t198, t336, t5023, t7193, t7669, t7673, t2159, t57, t606, t7214, dens_threshold, rho1, zeta_threshold);
        let t7683 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1207::<F>(t7599, t7682);
        let (t7687, t7690) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1208::<F>(t670, t7226, t7228, t7230, t7584, t7586, t118, t1310, t1453, t2127, t2163, t2165, t508, t569, t649, t651, t671, t6990, t6992, t6995, t7005, t7236, t7241, t7314, t7317, t7591, t7683);
    (t7662, t7666, t7669, t7673, t7677, t7683, t7687, t7690)
}
