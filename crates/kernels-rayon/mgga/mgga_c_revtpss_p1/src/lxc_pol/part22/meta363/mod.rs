//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1889;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1890;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta363(t3552: f64, t487: f64, t1208: f64, t3551: f64, t1209: f64, t3727: f64, t460: f64, t12295: f64, t1284: f64, t1204: f64, t3766: f64, t3153: f64, t3588: f64, t3555: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12654, t12657) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1889(t3552, t487, t1208, t3551);
        let (t12658, t12666, t12673, t12678, t12699, t12702) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1890(t12657, t487, t1209, t3727, t460, t12295, t1284, t3552, t1204, t3766);
        let (t12705, t12709) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1891(t3153, t3588, t3555, t3754);
    (t12654, t12657, t12658, t12666, t12673, t12678, t12699, t12702, t12705, t12709)
}
