//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1492;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1493;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1494;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta428(t1921: f64, t8330: f64, t1913: f64, t8349: f64, t31512: f64, t571: f64, t31463: f64, t575: f64, t1464: f64, t8416: f64, t1455: f64, t8433: f64, t116: f64, t31451: f64, t2212: f64, t5789: f64, t1513: f64, t2: f64, t670: f64, t8406: f64, t1459: f64, t1518: f64, t1916: f64, t21881: f64, t2207: f64, t22559: f64, t22565: f64, t22568: f64, t31234: f64, t31493: f64, t31505: f64, t31506: f64, t31509: f64, t31725: f64, t31731: f64, t31734: f64, t4292: f64, t572: f64, t5802: f64, t5920: f64, t6941: f64, t6945: f64, t8336: f64, t8342: f64, t8346: f64, t8421: f64, t31653: f64, t31027: f64, t31629: f64, t31636: f64, t31032: f64, t31643: f64, t117918: f64, t117920: f64, t117927: f64, t117936: f64, t117938: f64, t117940: f64, t117997: f64, t2357: f64, t31439: f64, t31443: f64, t36308: f64, t36315: f64, t31646: f64, t31649: f64, t109: f64, t1479: f64, t108: f64, t116912: f64, t31626: f64, t105875: f64, t117943: f64, t21872: f64, t21876: f64, t28036: f64, t31035: f64, t31287: f64, t31429: f64, t31433: f64, t4287: f64, t661: f64, t665: f64, t8258: f64, t8267: f64, t8311: f64, t8315: f64, t105880: f64, t117218: f64, t117544: f64, t117932: f64, t1509: f64, t21864: f64, t31149: f64, t31420: f64, t5907: f64, t5911: f64, t5915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t118091, t118094, t118099, t118106, t118108, t118110) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1491(t1921, t8330, t1913, t8349, t31512, t571, t31463, t575, t1464, t8416, t1455, t8433);
        let (t118203, t118374, t118629) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1492(t116, t31451, t2212, t5789, t1513, t2, t670, t8406, t1459, t1518, t1916, t21881, t2207, t22559, t22565, t22568, t31234, t31493, t31505, t31506, t31509, t31725, t31731, t31734, t4292, t572, t5802, t5920, t6941, t6945, t8336, t8342, t8346, t8421);
        let (t118630, t118655) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1493(t116, t31653, t31027, t31629, t31636, t31032, t31643, t117918, t117920, t117927, t117936, t117938, t117940, t117997, t1513, t2357, t31439, t31443, t36308, t36315);
        let t118688 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1494(t31032, t31646, t31649, t109, t1479, t108, t116912, t31626, t105875, t117943, t2, t21872, t21876, t28036, t31035, t31287, t31429, t31433, t4287, t661, t665, t8258, t8267, t8311, t8315);
        let t118728 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1495(t105880, t117218, t117544, t117932, t118374, t1509, t21864, t31035, t31149, t31287, t31420, t31433, t31439, t31443, t4287, t5907, t5911, t5915, t661, t665, t8258, t8267, t8311, t8315);
    (t118091, t118094, t118099, t118106, t118108, t118110, t118203, t118629, t118630, t118655, t118688, t118728)
}
