//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1950;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1951;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1952;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta558<F: Float>(t30031: F, t7296: F, t6846: F, t7264: F, t6880: F, t7271: F, t6856: F, t6876: F, t26017: F, t6850: F, t26028: F, t6871: F, t6884: F, t7252: F, t25983: F, t6864: F, t26003: F, t26011: F, t26013: F, t26022: F, t27921: F, t27953: F, t28873: F, t28874: F, t28885: F, t25970: F, t25976: F, t28872: F, t28877: F, t545: F, t2028: F, t1904: F, t2027: F, t25893: F, t25919: F, t25941: F, t25948: F, t25955: F, t27837: F, t27861: F, t27874: F, t27876: F, t27885: F, t27889: F, t27891: F, t27900: F, t27909: F, t30017: F, t30021: F, t6919: F, t7279: F, t7295: F, t7921: F) -> (F, F, F, F, F) {
        let (t30032, t30035, t30037, t30039, t30041, t30043, t30045) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1950::<F>(t30031, t7296, t6846, t7264, t6880, t7271, t6856, t6876, t26017, t6850, t26028, t6871);
        let t30054 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1951::<F>(t6884, t7252, t25983, t6864, t26003, t26011, t26013, t26022, t27921, t27953, t28873, t28874, t28885);
        let t30055 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1952::<F>(t25970, t25976, t28872, t28877, t30035, t30037, t30039, t30041, t30043, t30045, t30054);
        let (t30056, t30057, t30066) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1953::<F>(t30055, t545, t2028, t1904, t2027, t25893, t25919, t25941, t25948, t25955, t27837, t27861, t27874, t27876, t27885, t27889, t27891, t27900, t27909, t30017, t30021, t30032, t6919, t7279, t7295, t7921);
    (t30032, t30055, t30056, t30057, t30066)
}
