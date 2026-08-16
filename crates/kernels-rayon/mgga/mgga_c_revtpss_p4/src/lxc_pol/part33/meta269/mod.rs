//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1204;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1205;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1206;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1207;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1208;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta269(t1248: f64, t1287: f64, t7660: f64, t2150: f64, t473: f64, t7627: f64, t1204: f64, t1215: f64, t1295: f64, t2144: f64, t2149: f64, t2152: f64, t460: f64, t7602: f64, t7629: f64, t7632: f64, t7636: f64, t7639: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t7654: f64, t7659: f64, t2155: f64, t3801: f64, t33: f64, t265: f64, t502: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t5023: f64, t7193: f64, t2159: f64, t57: f64, t606: f64, t7214: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t7599: f64, t670: f64, t7226: f64, t7228: f64, t7230: f64, t7584: f64, t7586: f64, t118: f64, t1310: f64, t1453: f64, t2127: f64, t2163: f64, t2165: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t6990: f64, t6992: f64, t6995: f64, t7005: f64, t7236: f64, t7241: f64, t7314: f64, t7317: f64, t7591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7662, t7666, t7669) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1204(t1248, t1287, t7660, t2150, t473, t7627, t1204, t1215, t1295, t2144, t2149, t2152, t460, t7602, t7629, t7632, t7636, t7639, t7643, t7645, t7648, t7651, t7654, t7659);
        let t7673 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1205(t2155, t3801);
        let (t7677, t7682) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1206(t33, t265, t502, t1298, t1300, t198, t336, t5023, t7193, t7669, t7673, t2159, t57, t606, t7214, dens_threshold, rho1, zeta_threshold);
        let t7683 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1207(t7599, t7682);
        let (t7687, t7690) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1208(t670, t7226, t7228, t7230, t7584, t7586, t118, t1310, t1453, t2127, t2163, t2165, t508, t569, t649, t651, t671, t6990, t6992, t6995, t7005, t7236, t7241, t7314, t7317, t7591, t7683);
    (t7662, t7666, t7669, t7673, t7677, t7683, t7687, t7690)
}
