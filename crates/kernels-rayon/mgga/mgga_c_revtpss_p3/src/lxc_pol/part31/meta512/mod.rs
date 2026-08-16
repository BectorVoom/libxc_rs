//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1852;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1853;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1854;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1855;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1856;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta512(t4924: f64, t7111: f64, t1058: f64, t7801: f64, t1659: f64, t7125: f64, t1972: f64, t4797: f64, t4845: f64, t7117: f64, t4857: f64, t7131: f64, t25515: f64, t4890: f64, t3299: f64, t1028: f64, t1068: f64, t1665: f64, t1675: f64, t25490: f64, t25495: f64, t25529: f64, t25569: f64, t25577: f64, t4831: f64, t4854: f64, t4896: f64, t7132: f64, t3317: f64, t1671: f64, t25512: f64, t25522: f64, t25526: f64, t25535: f64, t25538: f64, t25580: f64, t4825: f64, t4869: f64, t4875: f64, t4887: f64, t4902: f64, t4907: f64, t4912: f64, t7122: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27460, t27462, t27464, t27467, t27471, t27479) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1852(t4924, t7111, t1058, t7801, t1659, t7125, t1972, t4797, t4845, t7117, t4857);
        let t27489 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1853(t1659, t7131);
        let t27492 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1854(t25515, t4890);
        let t27493 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1855(t27492, t3299);
        let t27496 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1856(t1028, t1068, t1665, t1675, t25490, t25495, t25529, t25569, t25577, t27471, t27479, t27489, t27493, t4831, t4854, t4896, t7117, t7132);
        let (t27498, t27518) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1857(t27492, t3317, t1671, t25512, t25522, t25526, t25535, t25538, t25580, t4825, t4869, t4875, t4887, t4902, t4907, t4912, t7111, t7122);
    (t27460, t27462, t27464, t27467, t27471, t27479, t27489, t27492, t27493, t27496, t27498, t27518)
}
