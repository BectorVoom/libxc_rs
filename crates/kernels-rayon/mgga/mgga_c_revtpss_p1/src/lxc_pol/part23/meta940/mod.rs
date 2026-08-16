//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta940 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3087;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3088;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3089;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta940(t20365: f64, t5079: f64, t16862: f64, t6449: f64, t20337: f64, t5087: f64, t1134: f64, t24312: f64, t3407: f64, t141: f64, t3417: f64, t81177: f64, t81186: f64, t81509: f64, t81511: f64, t81514: f64, t81516: f64, t81518: f64, t81521: f64, t24297: f64, t698: f64, t58225: f64, t68454: f64, t68456: f64, t68538: f64, t68540: f64, t68548: f64, t68550: f64, t68567: f64, t68583: f64, t68585: f64, t68590: f64, t1131: f64, t1150: f64, t81403: f64, t81418: f64, t81437: f64, t81472: f64, t81485: f64, t81506: f64, t24327: f64, t44012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81523, t81525, t81527, t81530, t81533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3087(t20365, t5079, t16862, t6449, t20337, t5087, t1134, t24312, t3407, t141, t3417, t81177);
        let (t81536, t81538) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3088(t141, t3417, t81186, t81509, t81511, t81514, t81516, t81518, t81521, t81523, t81525, t81527, t81530, t81533);
        let (t81539, t81552) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3089(t24297, t698, t58225, t68454, t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590);
        let (t81558, t81560) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3090(t1131, t1150, t81403, t81418, t81437, t81472, t81485, t81506, t81538, t81552, t24327, t44012);
    (t81523, t81525, t81527, t81530, t81533, t81536, t81539, t81558, t81560)
}
