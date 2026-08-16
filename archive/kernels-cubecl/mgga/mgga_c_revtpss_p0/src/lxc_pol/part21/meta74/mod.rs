//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta74 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk541;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk542;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk543;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk544;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk545;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk546;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta74<F: Float>(t5: F, t1466: F, t1497: F, t603: F, t91: F, t117: F, t1468: F, t100: F, t55: F, tau1: F, t108: F, t105: F, t109: F, t97: F, t114: F, t655: F, t653: F, t69: F, t508: F, t1469: F, t190: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1501, t1502) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk541::<F>(t5, t1466, t1497, t603, t91, t117);
        let t1504 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk542::<F>(t1468);
        let (t1505, t1507, t1509) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk543::<F>(t100, t1504, t55, tau1);
        let t1513 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk544::<F>(t108, t1509, t105, t109, t1505, t1507, t97);
        let (t1514, t1518) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk545::<F>(t114, t1513, t655, t653, t69);
        let t1519 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk546::<F>(t1518, t508);
        let t1522 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk547::<F>(t1469, t190);
    (t1501, t1502, t1504, t1505, t1507, t1509, t1513, t1514, t1518, t1519, t1522)
}
