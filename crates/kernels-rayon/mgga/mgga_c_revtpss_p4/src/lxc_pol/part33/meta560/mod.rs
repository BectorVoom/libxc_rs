//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta560 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1955;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta560(t30100: f64, t7301: f64, t1882: f64, t1903: f64, t543: f64, t25931: f64, t2030: f64, t213: f64, t25930: f64, t26040: f64, t26043: f64, t26058: f64, t26071: f64, t27837: f64, t27966: f64, t27969: f64, t27987: f64, t27990: f64, t27992: f64, t30071: f64, t30074: f64, t30082: f64, t30089: f64, t30096: f64, t561: f64, t6896: f64, t7279: f64, t7295: f64, t7917: f64, t7926: f64, t7930: f64, t30066: f64, t532: f64, t1450: f64, t2014: f64, t1868: f64, t1907: f64, t8717: f64, t25082: f64, t7732: f64, t7742: f64, t1936: f64, t6765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30101, t30105, t30106, t30109) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1955(t30100, t7301, t1882, t1903, t543, t25931, t2030, t213, t25930, t26040, t26043, t26058, t26071, t27837, t27966, t27969, t27987, t27990, t27992, t30071, t30074, t30082, t30089, t30096, t561, t6896, t7279, t7295, t7917, t7926, t7930);
        let (t30110, t30111, t30112, t30113, t30122, t30123, t30125, t30127, t30128) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1956(t30066, t30109, t532, t1450, t2014, t1868, t1907, t8717, t25082, t7732, t7742, t1936, t6765);
    (t30101, t30105, t30106, t30110, t30111, t30112, t30113, t30122, t30123, t30125, t30127, t30128)
}
