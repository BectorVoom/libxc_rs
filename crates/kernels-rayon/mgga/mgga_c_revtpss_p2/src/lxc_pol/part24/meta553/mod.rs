//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1644;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1645;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1646;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1647;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1648;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta553(t6157: f64, t6173: f64, t11409: f64, t15350: f64, t15406: f64, t19156: f64, t23706: f64, t23711: f64, t2943: f64, t2968: f64, t2970: f64, t41740: f64, t41742: f64, t6206: f64, t6209: f64, t64125: f64, t88023: f64, t88026: f64, t88028: f64, t88030: f64, t88034: f64, t88048: f64, t88050: f64, t88052: f64, t88054: f64, t954: f64, t11150: f64, t87145: f64, t128: f64, t904: f64, t5825: f64, t6092: f64, t2857: f64, t87107: f64, t22671: f64, t4578: f64, t41549: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t41296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88055, t88077) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1644(t6157, t6173, t11409, t15350, t15406, t19156, t23706, t23711, t2943, t2968, t2970, t41740, t41742, t6206, t6209, t64125, t88023, t88026, t88028, t88030, t88034, t88048, t88050, t88052, t88054, t954);
        let (t88083, t88085) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1645(t11150, t87145, t128, t904);
        let (t88087, t88089) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1646(t5825, t6092, t128, t904);
        let (t88091, t88093) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1647(t2857, t87107, t128, t904);
        let (t88095, t88097) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1648(t22671, t4578, t128, t904);
        let (t88100, t88102) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1649(t41549, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097, t41296, t87145);
    (t88055, t88077, t88083, t88085, t88087, t88089, t88091, t88093, t88095, t88097, t88100, t88102)
}
