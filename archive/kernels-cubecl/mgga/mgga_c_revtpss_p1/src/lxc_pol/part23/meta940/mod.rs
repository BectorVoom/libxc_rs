//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta940 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3087;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3088;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3089;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta940<F: Float>(t20365: F, t5079: F, t16862: F, t6449: F, t20337: F, t5087: F, t1134: F, t24312: F, t3407: F, t141: F, t3417: F, t81177: F, t81186: F, t81509: F, t81511: F, t81514: F, t81516: F, t81518: F, t81521: F, t24297: F, t698: F, t58225: F, t68454: F, t68456: F, t68538: F, t68540: F, t68548: F, t68550: F, t68567: F, t68583: F, t68585: F, t68590: F, t1131: F, t1150: F, t81403: F, t81418: F, t81437: F, t81472: F, t81485: F, t81506: F, t24327: F, t44012: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t81523, t81525, t81527, t81530, t81533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3087::<F>(t20365, t5079, t16862, t6449, t20337, t5087, t1134, t24312, t3407, t141, t3417, t81177);
        let (t81536, t81538) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3088::<F>(t141, t3417, t81186, t81509, t81511, t81514, t81516, t81518, t81521, t81523, t81525, t81527, t81530, t81533);
        let (t81539, t81552) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3089::<F>(t24297, t698, t58225, t68454, t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590);
        let (t81558, t81560) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3090::<F>(t1131, t1150, t81403, t81418, t81437, t81472, t81485, t81506, t81538, t81552, t24327, t44012);
    (t81523, t81525, t81527, t81530, t81533, t81536, t81539, t81558, t81560)
}
