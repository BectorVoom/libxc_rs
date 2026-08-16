//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1960;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1961;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1962;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1963;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1964;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta528<F: Float>(t27940: F, t5677: F, t26028: F, t5697: F, t5701: F, t5706: F, t5614: F, t7271: F, t5661: F, t7264: F, t25997: F, t5665: F, t1873: F, t26004: F, t5690: F, t7252: F, t25970: F, t25976: F, t26013: F, t26015: F, t27933: F, t27937: F, t27931: F, t225: F, t1904: F, t7242: F, t689: F, t786: F, t7911: F, t1364: F, t1398: F, t1903: F, t543: F, t25931: F, t2022: F, t3999: F, t14230: F, t1445: F, t213: F, t25930: F, t25955: F, t26040: F, t26043: F, t26051: F, t26055: F, t26058: F, t27837: F, t27868: F, t27909: F, t561: F, t5775: F, t7279: F, t7298: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27941, t27943, t27945, t27947, t27949, t27951, t27953) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1960::<F>(t27940, t5677, t26028, t5697, t5701, t5706, t5614, t7271, t5661, t7264, t25997, t5665);
        let t27959 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1961::<F>(t1873, t26004, t5690, t7252, t25970, t25976, t26013, t26015, t27933, t27937, t27941, t27943, t27945, t27947, t27949, t27951, t27953);
        let t27960 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1962::<F>(t27931, t27959);
        let (t27961, t27965, t27966, t27968, t27969, t27972, t27973) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1963::<F>(t225, t27960, t1904, t7242, t689, t786, t7911, t1364, t1398, t1903, t543, t25931);
        let t27980 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1964::<F>(t2022, t3999);
        let (t27981, t27984) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1965::<F>(t14230, t27980, t1445, t213, t25930, t25955, t26040, t26043, t26051, t26055, t26058, t27837, t27868, t27909, t27961, t27966, t27969, t27973, t561, t5775, t7279, t7298);
    (t27960, t27961, t27965, t27968, t27972, t27973, t27980, t27981, t27984)
}
