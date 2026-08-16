//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk992;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk993;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk994;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk995;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta221(t5826: f64, t70: f64, t1470: f64, t1486: f64, t2275: f64, t5819: f64, t48: f64, t5825: f64, t476: f64, t53: f64, t2282: f64, t60: f64, sigma2: f64, t1480: f64, t1483: f64, t2290: f64, t44: f64, t56: f64, t61: f64, t38: f64, t2299: f64, t633: f64, t2306: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t5820: f64, t71: f64, t85: f64, t5: f64, t1497: f64, t2247: f64, t4173: f64, t5812: f64, t5816: f64, t603: f64, t91: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5827, t5830, t5835, t5838, t5843, t5848, t5851) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk992(t5826, t70, t1470, t1486, t2275, t5819, t48, t5825, t476, t53, t2282, t60, sigma2);
        let (t5854, t5855) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk993(t1480, t1483, t2290, t44, t56, t5835, t5838, t5843, t5848, t5851, t61, t38);
        let (t5868, t5869, t5872) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk994(t2299, t5819, t5825, t633, t2306, t637, t77, t1471, t1487, t1494, t5820, t5827, t5830, t5855, t71, t85);
        let (t5876, t5877) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk995(t5, t1497, t2247, t4173, t5812, t5816, t5872, t603, t91, t117);
    (t5827, t5830, t5835, t5838, t5843, t5854, t5855, t5868, t5869, t5872, t5876, t5877)
}
