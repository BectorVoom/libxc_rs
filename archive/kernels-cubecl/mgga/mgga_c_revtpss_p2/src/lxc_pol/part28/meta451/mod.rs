//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta451 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1704;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1705;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1706;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1707;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1708;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta451<F: Float>(t12772: F, t5401: F, t3625: F, t1214: F, t5341: F, t5332: F, t3720: F, t1250: F, t5346: F, t16725: F, t5312: F, t16729: F, t1222: F, t12855: F, t12910: F, t13069: F, t17437: F, t17438: F, t17444: F, t17447: F, t17448: F, t1797: F, t3631: F, t3674: F, t140: F, t3698: F, t5047: F, t1012: F, t13026: F, t16715: F, t16720: F, t1774: F, t3601: F, t3611: F, t12809: F, t12882: F, t12887: F, t12893: F, t12895: F, t12900: F, t12902: F, t12905: F, t1263: F, t5245: F, t1122: F, t1042: F, t1234: F, t5390: F, t3704: F, t5293: F, t1121: F, t606: F, t17353: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t4186: F, t5296: F, t1469: F, t3584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17453, t17454, t17456, t17461, t17464, t17467) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1704::<F>(t12772, t5401, t3625, t1214, t5341, t5332, t3720, t1250, t5346, t16725, t5312, t16729);
        let t17470 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1705::<F>(t1222, t12855, t12910, t13069, t17437, t17438, t17444, t17447, t17448, t17453, t17456, t17461, t17464, t17467, t1797, t3631, t3674);
        let (t17474, t17476, t17479, t17482, t17483) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1706::<F>(t140, t3698, t5047, t1222, t1012, t13026, t16715, t16720, t5312, t1774, t3601, t3611);
        let t17493 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1707::<F>(t17483, t3720, t1222, t12809, t12882, t12887, t12893, t12895, t12900, t12902, t12905, t17474, t17476, t17479);
        let (t17502, t17505, t17509, t17514) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1708::<F>(t1263, t5245, t1122, t1042, t1234, t5390, t3704, t5293, t1121, t1214, t606, t1250);
        let (t17515, t17525, t17529, t17536, t17539) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1709::<F>(t17353, t17514, t1802, t3147, t3597, t3594, t1244, t1214, t4186, t5296, t1042, t1469, t3584);
    (t17454, t17470, t17482, t17493, t17502, t17505, t17509, t17515, t17525, t17529, t17536, t17539)
}
