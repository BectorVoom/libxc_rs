//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta228 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1340;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1341;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1342;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1343;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1344;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta228<F: Float>(t3523: F, t6555: F, t1196: F, t3546: F, t5044: F, t6423: F, t6427: F, t6431: F, t459: F, t1774: F, t1211: F, t1828: F, t1277: F, t3579: F, t1477: F, t476: F, t52: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6556, t6558, t6563, t6564) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1339::<F>(t3523, t6555, t1196, t3546, t5044, t6423, t6427, t6431, t459);
        let t6573 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1340::<F>(t1774);
        let t6574 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1341::<F>(t1211, t6573);
        let t6580 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1342::<F>(t1774, t1828, t1277);
        let t6587 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1343::<F>(t3579, t5044, t6423, t6427, t6431);
        let t6588 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1344::<F>(t1211, t6587);
        let (t6593, t6594) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1345::<F>(t1477, t476, t52, t475);
    (t6556, t6558, t6563, t6564, t6573, t6574, t6580, t6587, t6588, t6593, t6594)
}
