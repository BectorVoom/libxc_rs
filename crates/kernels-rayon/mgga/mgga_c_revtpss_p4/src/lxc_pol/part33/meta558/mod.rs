//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1950;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1951;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1952;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta558(t30031: f64, t7296: f64, t6846: f64, t7264: f64, t6880: f64, t7271: f64, t6856: f64, t6876: f64, t26017: f64, t6850: f64, t26028: f64, t6871: f64, t6884: f64, t7252: f64, t25983: f64, t6864: f64, t26003: f64, t26011: f64, t26013: f64, t26022: f64, t27921: f64, t27953: f64, t28873: f64, t28874: f64, t28885: f64, t25970: f64, t25976: f64, t28872: f64, t28877: f64, t545: f64, t2028: f64, t1904: f64, t2027: f64, t25893: f64, t25919: f64, t25941: f64, t25948: f64, t25955: f64, t27837: f64, t27861: f64, t27874: f64, t27876: f64, t27885: f64, t27889: f64, t27891: f64, t27900: f64, t27909: f64, t30017: f64, t30021: f64, t6919: f64, t7279: f64, t7295: f64, t7921: f64) -> (f64, f64, f64, f64, f64) {
        let (t30032, t30035, t30037, t30039, t30041, t30043, t30045) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1950(t30031, t7296, t6846, t7264, t6880, t7271, t6856, t6876, t26017, t6850, t26028, t6871);
        let t30054 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1951(t6884, t7252, t25983, t6864, t26003, t26011, t26013, t26022, t27921, t27953, t28873, t28874, t28885);
        let t30055 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1952(t25970, t25976, t28872, t28877, t30035, t30037, t30039, t30041, t30043, t30045, t30054);
        let (t30056, t30057, t30066) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1953(t30055, t545, t2028, t1904, t2027, t25893, t25919, t25941, t25948, t25955, t27837, t27861, t27874, t27876, t27885, t27889, t27891, t27900, t27909, t30017, t30021, t30032, t6919, t7279, t7295, t7921);
    (t30032, t30055, t30056, t30057, t30066)
}
