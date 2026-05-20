//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta44 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk330;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk331;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk332;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk333;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk334;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk335;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk336;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta44<F: Float>(t837: F, t879: F, t234: F, t860: F, t213: F, t820: F, t873: F, t878: F, t868: F, t783: F, t791: F, t862: F, t865: F, t261: F, t198: F, t207: F, t679: F, t704: F, t709: F, t718: F, t751: F, t754: F, t759: F, t764: F, t765: F, t775: F, t159: F, t675: F, t268: F, t271: F, t373: F, t631: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t880, t883, t886) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk330::<F>(t837, t879, t234, t860, t213, t820, t873, t878);
        let t887 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk331::<F>(t868, t886);
        let t890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk332::<F>(t213, t783, t791, t862, t865, t887);
        let t892 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk333::<F>(t261);
        let t895 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk334::<F>(t198, t207, t679, t704, t709, t718, t751, t754, t759, t764, t765, t775, t890, t892);
        let (t900, t902) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk335::<F>(t159, t675, t268, t271);
        let (t903, t904) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk336::<F>(t902, t159, t373);
        let t905 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk337::<F>(t631);
    (t880, t883, t886, t887, t890, t892, t895, t900, t902, t903, t904, t905)
}
