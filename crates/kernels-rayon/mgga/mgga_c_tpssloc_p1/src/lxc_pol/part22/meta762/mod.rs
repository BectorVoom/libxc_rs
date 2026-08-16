//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta762 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2564;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2565;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2566;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2567;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2568;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta762(t1117: f64, t11190: f64, t21724: f64, t3313: f64, t4781: f64, t5989: f64, t11424: f64, t21895: f64, t1147: f64, t21826: f64, t1128: f64, t21975: f64, t11185: f64, t11297: f64, t11365: f64, t1138: f64, t11415: f64, t1155: f64, t1157: f64, t15146: f64, t1695: f64, t18637: f64, t18644: f64, t18785: f64, t21836: f64, t21947: f64, t21952: f64, t3376: f64, t3401: f64, t4857: f64, t4858: f64, t51427: f64, t51730: f64, t6037: f64, t6069: f64, t6084: f64, t51246: f64, t1098: f64, t21988: f64, t1119: f64, t50834: f64, t51257: f64, t63291: f64, t63306: f64, t63308: f64, t63841: f64, t63843: f64, t63845: f64, t71333: f64, t71335: f64, t71337: f64, t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63893: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71391: f64, t63911: f64, t71144: f64, t71400: f64, t71403: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64, t71417: f64, t71420: f64, t71423: f64, t71426: f64, t50846: f64, t51271: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71850, t71853, t71855, t71860, t71863) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2564(t1117, t11190, t21724, t3313, t4781, t5989, t11424, t21895, t1147, t21826, t1128, t21975);
        let (t71867, t71868) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2565(t11185, t21724, t11297, t11365, t1138, t11415, t1155, t1157, t15146, t1695, t18637, t18644, t18785, t21836, t21947, t21952, t3376, t3401, t4857, t4858, t51427, t51730, t6037, t6069, t6084, t71850, t71853, t71855, t71860, t71863);
        let (t71876, t71879, t71902) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2566(t51246, t5989, t1098, t21988, t1119, t50834, t51257, t63291, t63306, t63308, t63841, t63843, t63845, t71333, t71335, t71337);
        let t71915 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2567(t63332, t63334, t63336, t63886, t63888, t63893, t71124, t71130, t71135, t71140, t71142, t71391);
        let t71929 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2568(t63911, t71144, t71400, t71403, t71406, t71408, t71411, t71414, t71417, t71420, t71423, t71426);
        let t71941 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2569(t50846, t51271, t71146, t71150, t71152, t71154, t71156, t71160, t71166, t71170, t71174, t71179);
    (t71850, t71853, t71855, t71867, t71868, t71876, t71879, t71902, t71915, t71929, t71941)
}
