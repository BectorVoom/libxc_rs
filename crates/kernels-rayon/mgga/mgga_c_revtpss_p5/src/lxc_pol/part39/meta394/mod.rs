//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta394 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1422;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1423;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1424;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1425;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1426;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta394(t12772: f64, t5401: f64, t3625: f64, t1214: f64, t5341: f64, t5332: f64, t3720: f64, t1250: f64, t5346: f64, t16725: f64, t5312: f64, t16729: f64, t1222: f64, t12855: f64, t12910: f64, t13069: f64, t17437: f64, t17438: f64, t17444: f64, t17447: f64, t17448: f64, t1797: f64, t3631: f64, t3674: f64, t140: f64, t3698: f64, t5047: f64, t1012: f64, t13026: f64, t16715: f64, t16720: f64, t1774: f64, t3601: f64, t3611: f64, t12809: f64, t12882: f64, t12887: f64, t12893: f64, t12895: f64, t12900: f64, t12902: f64, t12905: f64, t1263: f64, t5245: f64, t1122: f64, t1042: f64, t1234: f64, t5390: f64, t3704: f64, t5293: f64, t1121: f64, t606: f64, t17353: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64, t4186: f64, t5296: f64, t1469: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17453, t17454, t17456, t17461, t17464, t17467) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1422(t12772, t5401, t3625, t1214, t5341, t5332, t3720, t1250, t5346, t16725, t5312, t16729);
        let t17470 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1423(t1222, t12855, t12910, t13069, t17437, t17438, t17444, t17447, t17448, t17453, t17456, t17461, t17464, t17467, t1797, t3631, t3674);
        let (t17474, t17476, t17479, t17482, t17483) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1424(t140, t3698, t5047, t1222, t1012, t13026, t16715, t16720, t5312, t1774, t3601, t3611);
        let t17493 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1425(t17483, t3720, t1222, t12809, t12882, t12887, t12893, t12895, t12900, t12902, t12905, t17474, t17476, t17479);
        let (t17502, t17505, t17509, t17514) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1426(t1263, t5245, t1122, t1042, t1234, t5390, t3704, t5293, t1121, t1214, t606, t1250);
        let (t17515, t17525, t17529, t17536, t17539) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1427(t17353, t17514, t1802, t3147, t3597, t3594, t1244, t1214, t4186, t5296, t1042, t1469, t3584);
    (t17454, t17470, t17482, t17493, t17502, t17505, t17509, t17515, t17525, t17529, t17536, t17539)
}
