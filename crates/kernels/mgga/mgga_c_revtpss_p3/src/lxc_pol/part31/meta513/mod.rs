//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta513 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1858;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1859;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1860;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1861;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1862;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1863;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta513<F: Float>(t1967: F, t816: F, t1014: F, t65: F, t4579: F, t3252: F, t4574: F, t3204: F, t7131: F, t4817: F, t7132: F, t25517: F, t25543: F, t25551: F, t25557: F, t25560: F, t25564: F, t4783: F, t4788: F, t4839: F, t1047: F, t1656: F, t25498: F, t25539: F, t27448: F, t27450: F, t27460: F, t27462: F, t27464: F, t27467: F, t27496: F, t27518: F, t375: F, t4803: F, t4808: F, t225: F, t385: F, t7810: F, t994: F, t1000: F, t25461: F, t25476: F, t25611: F, t25629: F, t27412: F, t27415: F, t27419: F, t27423: F, t27427: F, t27433: F, t27437: F, t27441: F, t27445: F, t342: F, t4947: F, t7140: F, t7144: F, t7153: F, t7159: F, t7818: F, t7822: F, t999: F, t7145: F, t1976: F, t4746: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t27526 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1858::<F>(t1967, t816);
        let (t27527, t27528, t27531, t27532, t27536) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1859::<F>(t1014, t65, t4579, t3252, t4574, t3204, t7131);
        let (t27539, t27541) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1860::<F>(t4817, t7132, t25517, t25543, t25551, t25557, t25560, t25564, t27526, t27528, t27532, t27536, t4783, t4788, t4839);
        let t27543 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1861::<F>(t1047, t1656, t25498, t25539, t27448, t27450, t27460, t27462, t27464, t27467, t27496, t27518, t27541, t375, t4803, t4808, t7132);
        let (t27545, t27550) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1862::<F>(t225, t27543, t385, t7810, t994);
        let t27553 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1863::<F>(t1000, t25461, t25476, t25611, t25629, t27412, t27415, t27419, t27423, t27427, t27433, t27437, t27441, t27445, t27545, t27550, t342, t4947, t7140, t7144, t7153, t7159, t7818, t7822);
        let (t27557, t27568) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1864::<F>(t7810, t999, t7145, t1976, t4746);
    (t27526, t27527, t27531, t27536, t27539, t27543, t27545, t27550, t27553, t27557, t27568)
}
