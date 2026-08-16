//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta325 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1606;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1607;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1608;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1609;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1610;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1611;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1612;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta325<F: Float>(t1096: F, t3270: F, t11121: F, t1071: F, t3046: F, t268: F, t271: F, t7021: F, t2435: F, t907: F, t2854: F, t689: F, t2859: F, t2863: F, t159: F, t3181: F, t2851: F, t631: F, t10356: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11122, t11123, t11128, t11132) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1606::<F>(t1096, t3270, t11121, t1071, t3046, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1607::<F>(t11132, t2435, t907);
        let t11136 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1608::<F>(t2854, t689);
        let t11138 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1609::<F>(t2859, t689);
        let t11140 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1610::<F>(t2863, t689);
        let (t11142, t11144) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1611::<F>(t159, t3181, t2851, t631);
        let t11145 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1612::<F>(t10356, t11144);
    (t11122, t11123, t11128, t11132, t11133, t11134, t11136, t11138, t11140, t11142, t11144, t11145)
}
