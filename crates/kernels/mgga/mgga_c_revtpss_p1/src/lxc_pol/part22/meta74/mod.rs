//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk539;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk540;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk541;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk542;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk543;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta74<F: Float>(t1470: F, t70: F, t1469: F, t48: F, t51: F, t53: F, rho1: F, sigma2: F, t60: F, t44: F, t56: F, t61: F, t626: F, t38: F, t633: F, t637: F, t77: F, t71: F, t85: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1471 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk539::<F>(t1470, t70);
        let (t1474, t1477, t1479, t1480) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk540::<F>(t1469, t48, t51, t53, rho1, sigma2);
        let (t1483, t1486) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk541::<F>(t1469, t60, t1474, t1480, t44, t56, t61, t626);
        let t1487 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk542::<F>(t1486, t38);
        let (t1490, t1491, t1494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk543::<F>(t1469, t633, t637, t77);
        let t1497 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk544::<F>(t1471, t1487, t1494, t71, t85);
    (t1471, t1474, t1477, t1479, t1480, t1483, t1486, t1487, t1490, t1491, t1494, t1497)
}
