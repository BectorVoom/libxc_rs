//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1071;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1072;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1073;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta170(t1389: f64, t2713: f64, t3964: f64, t1414: f64, t3889: f64, t828: f64, t2668: f64, t550: f64, t816: f64, t1379: f64, t1408: f64, t2482: f64, t27: f64, t136: f64, t1413: f64, t1353: f64, t221: f64, t247: f64, t2682: f64, t548: f64, t820: f64, t843: f64, t1416: f64, t1386: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3967, t3970, t3976, t3978) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1071(t1389, t2713, t3964, t1414, t3889, t828, t2668, t550, t816, t1379, t1408, t2482, t27);
        let t3979 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1072(t136, t1413);
        let (t3981, t3982, t3987, t3989) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1073(t1353, t221, t3979, t3978, t247, t2682, t550, t548, t1408, t820, t843);
        let (t3990, t3992) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1074(t1416, t3989, t1386, t240);
    (t3967, t3970, t3976, t3978, t3979, t3981, t3982, t3987, t3989, t3990, t3992)
}
