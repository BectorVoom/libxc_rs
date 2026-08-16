//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1866;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1867;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1868;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1869;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta522(t27940: f64, t5677: f64, t26028: f64, t5697: f64, t5701: f64, t5706: f64, t5614: f64, t7271: f64, t5661: f64, t7264: f64, t25997: f64, t5665: f64, t1873: f64, t26004: f64, t5690: f64, t7252: f64, t25970: f64, t25976: f64, t26013: f64, t26015: f64, t27933: f64, t27937: f64, t27931: f64, t225: f64, t1904: f64, t7242: f64, t689: f64, t786: f64, t7911: f64, t1364: f64, t1398: f64, t1903: f64, t543: f64, t25931: f64, t2022: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27941, t27943, t27945, t27947, t27949, t27951, t27953) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1866(t27940, t5677, t26028, t5697, t5701, t5706, t5614, t7271, t5661, t7264, t25997, t5665);
        let (t27955, t27959) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1867(t1873, t26004, t5690, t7252, t25970, t25976, t26013, t26015, t27933, t27937, t27941, t27943, t27945, t27947, t27949, t27951, t27953);
        let t27960 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1868(t27931, t27959);
        let (t27961, t27965, t27966, t27968, t27969, t27972, t27973) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1869(t225, t27960, t1904, t7242, t689, t786, t7911, t1364, t1398, t1903, t543, t25931);
        let t27980 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1870(t2022, t3999);
    (t27953, t27955, t27960, t27961, t27965, t27966, t27968, t27969, t27972, t27973, t27980)
}
