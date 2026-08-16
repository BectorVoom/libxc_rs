//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta383 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1806;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1807;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1808;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1809;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1810;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta383<F: Float>(t12295: F, t12351: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t12344: F, t12347: F, t12354: F, t12531: F, t1188: F, t1178: F, t3519: F, t439: F, t3522: F, t447: F, t12487: F, t1161: F, t1180: F, t1189: F, t12429: F, t12431: F, t12465: F, t12470: F, t12473: F, t12476: F, t12481: F, t12486: F, t12488: F, t12491: F, t12494: F, t12497: F, t12501: F, t12504: F, t12508: F, t12511: F, t12514: F, t3452: F, t3454: F, t3477: F, t3491: F, t3496: F, t3498: F, t3516: F, t3521: F, t3524: F, t12426: F, t300: F, t12224: F, t12233: F, t12237: F, t12240: F, t12242: F, t12245: F, t12251: F, t12360: F, t12363: F, t12366: F, t12381: F, t12395: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12542, t12543, t12546) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1805::<F>(t12295, t12351, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320, t12344, t12347, t12354);
        let t12547 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1806::<F>(t12531, t12546);
        let (t12548, t12552) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1807::<F>(t1188, t12547, t1178, t3519);
        let t12553 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1808::<F>(t12552, t439);
        let t12555 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1809::<F>(t3522, t447);
        let (t12556, t12559) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1810::<F>(t12487, t12555, t1161, t1180, t1189, t12429, t12431, t12465, t12470, t12473, t12476, t12481, t12486, t12488, t12491, t12494, t12497, t12501, t12504, t12508, t12511, t12514, t12548, t12553, t3452, t3454, t3477, t3491, t3496, t3498, t3516, t3521, t3524);
        let (t12561, t12562) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1811::<F>(t12426, t12559, t300, t12224, t12233, t12237, t12240, t12242, t12245, t12251, t12360, t12363, t12366, t12381, t12395);
    (t12542, t12543, t12547, t12548, t12552, t12553, t12555, t12556, t12561, t12562)
}
