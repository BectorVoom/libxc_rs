//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta428 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1492;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1493;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1494;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta428<F: Float>(t1921: F, t8330: F, t1913: F, t8349: F, t31512: F, t571: F, t31463: F, t575: F, t1464: F, t8416: F, t1455: F, t8433: F, t116: F, t31451: F, t2212: F, t5789: F, t1513: F, t2: F, t670: F, t8406: F, t1459: F, t1518: F, t1916: F, t21881: F, t2207: F, t22559: F, t22565: F, t22568: F, t31234: F, t31493: F, t31505: F, t31506: F, t31509: F, t31725: F, t31731: F, t31734: F, t4292: F, t572: F, t5802: F, t5920: F, t6941: F, t6945: F, t8336: F, t8342: F, t8346: F, t8421: F, t31653: F, t31027: F, t31629: F, t31636: F, t31032: F, t31643: F, t117918: F, t117920: F, t117927: F, t117936: F, t117938: F, t117940: F, t117997: F, t2357: F, t31439: F, t31443: F, t36308: F, t36315: F, t31646: F, t31649: F, t109: F, t1479: F, t108: F, t116912: F, t31626: F, t105875: F, t117943: F, t21872: F, t21876: F, t28036: F, t31035: F, t31287: F, t31429: F, t31433: F, t4287: F, t661: F, t665: F, t8258: F, t8267: F, t8311: F, t8315: F, t105880: F, t117218: F, t117544: F, t117932: F, t1509: F, t21864: F, t31149: F, t31420: F, t5907: F, t5911: F, t5915: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t118091, t118094, t118099, t118106, t118108, t118110) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1491::<F>(t1921, t8330, t1913, t8349, t31512, t571, t31463, t575, t1464, t8416, t1455, t8433);
        let (t118203, t118374, t118629) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1492::<F>(t116, t31451, t2212, t5789, t1513, t2, t670, t8406, t1459, t1518, t1916, t21881, t2207, t22559, t22565, t22568, t31234, t31493, t31505, t31506, t31509, t31725, t31731, t31734, t4292, t572, t5802, t5920, t6941, t6945, t8336, t8342, t8346, t8421);
        let (t118630, t118655) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1493::<F>(t116, t31653, t31027, t31629, t31636, t31032, t31643, t117918, t117920, t117927, t117936, t117938, t117940, t117997, t1513, t2357, t31439, t31443, t36308, t36315);
        let t118688 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1494::<F>(t31032, t31646, t31649, t109, t1479, t108, t116912, t31626, t105875, t117943, t2, t21872, t21876, t28036, t31035, t31287, t31429, t31433, t4287, t661, t665, t8258, t8267, t8311, t8315);
        let t118728 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1495::<F>(t105880, t117218, t117544, t117932, t118374, t1509, t21864, t31035, t31149, t31287, t31420, t31433, t31439, t31443, t4287, t5907, t5911, t5915, t661, t665, t8258, t8267, t8311, t8315);
    (t118091, t118094, t118099, t118106, t118108, t118110, t118203, t118629, t118630, t118655, t118688, t118728)
}
