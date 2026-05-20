//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1009;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1010;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1011;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1012;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1013;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1014;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta159<F: Float>(t1214: F, t1263: F, t1122: F, t1042: F, t1209: F, t1284: F, t3624: F, t482: F, t66: F, t828: F, t1248: F, t1250: F, t1222: F, t1235: F, t1238: F, t1252: F, t3663: F, t3667: F, t3671: F, t3674: F, t3679: F, t3684: F, t3686: F, t3689: F, t3694: F, t3701: F, t3705: F, t3708: F, t3711: F, t3660: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3712, t3713, t3714) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1009::<F>(t1214, t1263, t1122, t1042);
        let (t3717, t3718) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1010::<F>(t1209, t1284, t3624);
        let t3719 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1011::<F>(t482, t66);
        let t3720 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1012::<F>(t3719, t828);
        let (t3721, t3722, t3723) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1013::<F>(t1214, t1248, t1250, t3720);
        let t3726 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1014::<F>(t1222, t1235, t1238, t1252, t3663, t3667, t3671, t3674, t3679, t3684, t3686, t3689, t3694, t3701, t3705, t3708, t3711, t3714, t3718, t3723);
        let t3727 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1015::<F>(t3660, t3726);
    (t3712, t3713, t3714, t3717, t3718, t3719, t3720, t3721, t3722, t3723, t3727)
}
