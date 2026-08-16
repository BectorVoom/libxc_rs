//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1827;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1828;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1829;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta388(t1294: f64, t3790: f64, t3737: f64, t1284: f64, t3552: f64, t1204: f64, t3766: f64, t3153: f64, t3588: f64, t5480: f64, t3555: f64, t3754: f64, t1248: f64, t5464: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12695, t12696, t12699) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1827(t1294, t3790, t3737, t1284, t3552);
        let (t12702, t12705, t12706, t12709) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1828(t1204, t3766, t3153, t3588, t5480, t3555, t3754);
        let t12712 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1829(t1248, t3153);
        let (t12713, t12714, t12717) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1830(t3588, t5464, t12712, t3566, t3754);
    (t12695, t12696, t12699, t12702, t12705, t12706, t12709, t12712, t12713, t12714, t12717)
}
