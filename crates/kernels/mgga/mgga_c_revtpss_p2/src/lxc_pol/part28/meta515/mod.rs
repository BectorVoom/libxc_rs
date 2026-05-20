//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta515 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1926;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1927;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1928;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1929;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1930;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta515<F: Float>(t4878: F, t7121: F, t4924: F, t7111: F, t1058: F, t7801: F, t1659: F, t7125: F, t1972: F, t4797: F, t4845: F, t7117: F, t4857: F, t7131: F, t25515: F, t4890: F, t3299: F, t1028: F, t1068: F, t1665: F, t1675: F, t25490: F, t25495: F, t25529: F, t25569: F, t25577: F, t4831: F, t4854: F, t4896: F, t7132: F, t3317: F, t1671: F, t25512: F, t25522: F, t25526: F, t25535: F, t25538: F, t25580: F, t4825: F, t4869: F, t4875: F, t4887: F, t4902: F, t4907: F, t4912: F, t7122: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27450, t27460, t27462, t27464, t27467, t27471) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1926::<F>(t4878, t7121, t4924, t7111, t1058, t7801, t1659, t7125, t1972, t4797, t4845, t7117);
        let (t27479, t27489, t27492) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1927::<F>(t1972, t4857, t1659, t7131, t25515, t4890);
        let t27493 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1928::<F>(t27492, t3299);
        let t27496 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1929::<F>(t1028, t1068, t1665, t1675, t25490, t25495, t25529, t25569, t25577, t27471, t27479, t27489, t27493, t4831, t4854, t4896, t7117, t7132);
        let t27498 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1930::<F>(t27492, t3317);
        let t27518 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1931::<F>(t1671, t25512, t25522, t25526, t25535, t25538, t25580, t27498, t4825, t4869, t4875, t4887, t4902, t4907, t4912, t7111, t7122);
    (t27450, t27460, t27462, t27464, t27467, t27479, t27489, t27492, t27493, t27496, t27498, t27518)
}
