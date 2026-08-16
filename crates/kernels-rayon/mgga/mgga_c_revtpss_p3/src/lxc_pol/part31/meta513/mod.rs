//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1858;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1859;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1860;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1861;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1862;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1863;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta513(t1967: f64, t816: f64, t1014: f64, t65: f64, t4579: f64, t3252: f64, t4574: f64, t3204: f64, t7131: f64, t4817: f64, t7132: f64, t25517: f64, t25543: f64, t25551: f64, t25557: f64, t25560: f64, t25564: f64, t4783: f64, t4788: f64, t4839: f64, t1047: f64, t1656: f64, t25498: f64, t25539: f64, t27448: f64, t27450: f64, t27460: f64, t27462: f64, t27464: f64, t27467: f64, t27496: f64, t27518: f64, t375: f64, t4803: f64, t4808: f64, t225: f64, t385: f64, t7810: f64, t994: f64, t1000: f64, t25461: f64, t25476: f64, t25611: f64, t25629: f64, t27412: f64, t27415: f64, t27419: f64, t27423: f64, t27427: f64, t27433: f64, t27437: f64, t27441: f64, t27445: f64, t342: f64, t4947: f64, t7140: f64, t7144: f64, t7153: f64, t7159: f64, t7818: f64, t7822: f64, t999: f64, t7145: f64, t1976: f64, t4746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t27526 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1858(t1967, t816);
        let (t27527, t27528, t27531, t27532, t27536) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1859(t1014, t65, t4579, t3252, t4574, t3204, t7131);
        let (t27539, t27541) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1860(t4817, t7132, t25517, t25543, t25551, t25557, t25560, t25564, t27526, t27528, t27532, t27536, t4783, t4788, t4839);
        let t27543 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1861(t1047, t1656, t25498, t25539, t27448, t27450, t27460, t27462, t27464, t27467, t27496, t27518, t27541, t375, t4803, t4808, t7132);
        let (t27545, t27550) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1862(t225, t27543, t385, t7810, t994);
        let t27553 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1863(t1000, t25461, t25476, t25611, t25629, t27412, t27415, t27419, t27423, t27427, t27433, t27437, t27441, t27445, t27545, t27550, t342, t4947, t7140, t7144, t7153, t7159, t7818, t7822);
        let (t27557, t27568) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1864(t7810, t999, t7145, t1976, t4746);
    (t27526, t27527, t27531, t27536, t27539, t27543, t27545, t27550, t27553, t27557, t27568)
}
