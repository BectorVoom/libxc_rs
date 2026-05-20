//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta553 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1644;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1645;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1646;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1647;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1648;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta553<F: Float>(t6157: F, t6173: F, t11409: F, t15350: F, t15406: F, t19156: F, t23706: F, t23711: F, t2943: F, t2968: F, t2970: F, t41740: F, t41742: F, t6206: F, t6209: F, t64125: F, t88023: F, t88026: F, t88028: F, t88030: F, t88034: F, t88048: F, t88050: F, t88052: F, t88054: F, t954: F, t11150: F, t87145: F, t128: F, t904: F, t5825: F, t6092: F, t2857: F, t87107: F, t22671: F, t4578: F, t41549: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t41296: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88055, t88077) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1644::<F>(t6157, t6173, t11409, t15350, t15406, t19156, t23706, t23711, t2943, t2968, t2970, t41740, t41742, t6206, t6209, t64125, t88023, t88026, t88028, t88030, t88034, t88048, t88050, t88052, t88054, t954);
        let (t88083, t88085) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1645::<F>(t11150, t87145, t128, t904);
        let (t88087, t88089) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1646::<F>(t5825, t6092, t128, t904);
        let (t88091, t88093) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1647::<F>(t2857, t87107, t128, t904);
        let (t88095, t88097) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1648::<F>(t22671, t4578, t128, t904);
        let (t88100, t88102) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1649::<F>(t41549, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097, t41296, t87145);
    (t88055, t88077, t88083, t88085, t88087, t88089, t88091, t88093, t88095, t88097, t88100, t88102)
}
