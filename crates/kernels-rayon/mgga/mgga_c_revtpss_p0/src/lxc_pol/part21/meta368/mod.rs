//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1744;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1745;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1746;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1747;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta368(t3059: f64, t3291: f64, t4980: f64, t994: f64, t3151: f64, t999: f64, t3304: f64, t4995: f64, t3318: f64, t1043: f64, t3153: f64, t3133: f64, t4982: f64, t1071: f64, t1089: f64, t3046: f64, t3286: f64, t3057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12119, t12122) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1744(t3059, t3291, t4980, t994);
        let (t12123, t12124, t12127) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1745(t3151, t999, t3304, t4995, t994);
        let (t12128, t12131) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1746(t12123, t3318, t1043, t3153);
        let (t12132, t12133, t12137, t12143, t12146) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1747(t3133, t4982, t12131, t1071, t1089, t999, t3046, t3286);
        let t12149 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1748(t3057, t3286);
    (t12119, t12122, t12124, t12127, t12128, t12131, t12132, t12133, t12137, t12143, t12146, t12149)
}
