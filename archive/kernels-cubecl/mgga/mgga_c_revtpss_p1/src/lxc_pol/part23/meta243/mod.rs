//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta243 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1412;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1413;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1414;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1415;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1416;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1417;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1418;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta243<F: Float>(t2564: F, t2567: F, t268: F, t675: F, t30: F, t525: F, t2: F, t22: F, t33: F, t527: F, t2490: F, t737: F, t2492: F, t744: F, t185: F, t2494: F, t1340: F, t2516: F, t4038: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F, t738: F, t745: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t9333 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1412::<F>(t2564, t2567, t268, t675);
        let (t9335, t9342) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1413::<F>(t30, t525, t2, t22);
        let (t9350, t9367) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1414::<F>(t33, t527, t2490, t737);
        let t9368 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1415::<F>(t2492, t744);
        let t9371 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1416::<F>(t185, t2494);
        let t9372 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1417::<F>(t9367, t9368, t9371);
        let (t9374, t9375, t9385) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1418::<F>(t1340, t9372, t2516, t4038, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
        let t9387 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1419::<F>(t738, t745, t9385);
    (t9333, t9335, t9342, t9350, t9367, t9368, t9371, t9372, t9374, t9375, t9385, t9387)
}
