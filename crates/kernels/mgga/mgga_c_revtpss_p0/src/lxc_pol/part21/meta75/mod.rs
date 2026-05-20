//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta75 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk548;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk549;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk550;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk551;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta75<F: Float>(t45: F, t57: F, t1522: F, t706: F, t1469: F, t78: F, t81: F, zeta_threshold: F, t150: F, t190: F, t162: F, t187: F, t766: F, t770: F, t124: F, t800: F, t225: F, t679: F, t704: F, t751: F, t759: F, t764: F, t832: F, t227: F, t229: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1524, t1531) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk548::<F>(t45, t57, t1522, t706, t1469, t78, t81, zeta_threshold);
        let (t1532, t1533, t1534, t1536, t1544) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk549::<F>(t45, t57, t150, t1531, t190, t162, t187, t1469, t766, t770, zeta_threshold);
        let t1548 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk550::<F>(t124, t1544);
        let (t1549, t1553) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk551::<F>(t1548, t800, t1524, t1533, t1536, t225, t679, t704, t751, t759, t764);
        let (t1555, t1558) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk552::<F>(t1544, t832, t1553, t227, t229);
    (t1524, t1531, t1532, t1533, t1534, t1536, t1544, t1548, t1549, t1553, t1555, t1558)
}
