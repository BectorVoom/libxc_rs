//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1891;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1892;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1893;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1894;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta524<F: Float>(t27940: F, t5677: F, t26028: F, t5697: F, t5701: F, t5706: F, t5614: F, t7271: F, t5661: F, t7264: F, t25997: F, t5665: F, t1873: F, t26004: F, t5690: F, t7252: F, t25970: F, t25976: F, t26013: F, t26015: F, t27933: F, t27937: F, t27931: F, t225: F, t1904: F, t7242: F, t689: F, t786: F, t7911: F, t1364: F, t1398: F, t1903: F, t543: F, t25931: F, t2022: F, t3999: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t27941, t27943, t27945, t27947, t27949, t27951, t27953) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1891::<F>(t27940, t5677, t26028, t5697, t5701, t5706, t5614, t7271, t5661, t7264, t25997, t5665);
        let (t27955, t27959) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1892::<F>(t1873, t26004, t5690, t7252, t25970, t25976, t26013, t26015, t27933, t27937, t27941, t27943, t27945, t27947, t27949, t27951, t27953);
        let t27960 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1893::<F>(t27931, t27959);
        let (t27961, t27965, t27966, t27968, t27969, t27972, t27973) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1894::<F>(t225, t27960, t1904, t7242, t689, t786, t7911, t1364, t1398, t1903, t543, t25931);
        let t27980 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1895::<F>(t2022, t3999);
    (t27953, t27955, t27960, t27961, t27965, t27966, t27968, t27969, t27972, t27973, t27980)
}
