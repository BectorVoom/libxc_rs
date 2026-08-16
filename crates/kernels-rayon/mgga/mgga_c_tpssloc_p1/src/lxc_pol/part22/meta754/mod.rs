//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2533;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2534;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2535;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2536;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta754(t449: f64, t71289: f64, t71308: f64, t1671: f64, t63750: f64, t18686: f64, t4782: f64, t14845: f64, t6021: f64, t18835: f64, t4740: f64, t11310: f64, t11350: f64, t1136: f64, t1155: f64, t15171: f64, t15225: f64, t18612: f64, t18616: f64, t18640: f64, t18786: f64, t4835: f64, t51382: f64, t51389: f64, t51727: f64, t6052: f64, t6084: f64, t71095: f64, t71097: f64, t71217: f64, t71245: f64, t136: f64, t3297: f64, t71138: f64, t21746: f64, t699: f64, t21750: f64, t50827: f64, t50834: f64, t63291: f64, t63306: f64, t63308: f64, t63841: f64, t63843: f64, t63845: f64, t51058: f64, t63332: f64, t63334: f64, t63336: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71144: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t43816: f64, t43820: f64, t51073: f64, t51082: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t71183: f64, t71187: f64, t71191: f64, t71195: f64, t71199: f64, t71203: f64, t71206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71310, t71313, t71315, t71317, t71319, t71322) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2533(t449, t71289, t71308, t1671, t63750, t18686, t4782, t14845, t6021, t18835, t4740, t11310, t11350, t1136, t1155, t15171, t15225, t18612, t18616, t18640, t18786, t4835, t51382, t51389, t51727, t6052, t6084, t71095, t71097, t71217, t71245);
        let (t71333, t71335, t71337, t71343) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2534(t136, t3297, t71138, t21746, t699, t21750, t50827, t50834, t63291, t63306, t63308, t63841, t63843, t63845);
        let t71371 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2535(t50834, t51058, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let t71389 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2536(t43816, t43820, t51073, t51082, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
    (t71310, t71313, t71315, t71317, t71319, t71322, t71333, t71335, t71337, t71343, t71371, t71389)
}
