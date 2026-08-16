//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1093;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1094;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1095;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1096;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1097;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1099;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1100;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta306(t601: f64, t9238: f64, t85: f64, t24: f64, t2241: f64, t2307: f64, t10276: f64, t73: f64, t2244: f64, t2250: f64, t16: f64, t39031: f64, t25: f64, t28: f64, zeta_threshold: f64, t11152: f64, t76: f64, t2251: f64, t2252: f64, t2255: f64, t2283: f64, t2284: f64, t2291: f64, t2298: f64, t2304: f64, t608: f64, t609: f64, t629: f64, t634: f64, t638: f64, t642: f64, t66: f64, t72: f64, t80: f64, t9258: f64, t9263: f64, t9268: f64, t9312: f64, t9313: f64, t9321: f64, t9324: f64, t9330: f64, t9333: f64, t9339: f64, t41: f64, t42: f64, t53: f64, t54: f64, t1028: f64, t36: f64, t9576: f64, t2262: f64, t2267: f64, t2268: f64, t2271: f64, t2274: f64, t39: f64, t43: f64, t44: f64, t51: f64, t55: f64, t615: f64, t618: f64, t9277: f64, t9287: f64, t9289: f64, t9292: f64, t9293: f64, t9296: f64, t9300: f64, t9304: f64, sigma0: f64, t1864: f64, t2245: f64, t31: f64, t33: f64, t607: f64, t628: f64, t65: f64, t6509: f64, t67: f64, t9247: f64, t9248: f64, t9251: f64, t9259: f64, t9260: f64, t2235: f64, t2240: f64, t39030: f64, t39032: f64, t39034: f64, t39036: f64, t39038: f64, t39040: f64, t39043: f64, t39046: f64, t39049: f64, t605: f64, t645: f64, t86: f64, t9228: f64, t9231: f64, t9239: f64, t9240: f64, t9243: f64, t9342: f64, t5: f64, t112: f64, t2363: f64, t111: f64, t9346: f64, t2405: f64, t2420: f64, t702: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39054, t39063, t39064, t39070, t39096, t39097) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1093(t601, t9238, t85, t24, t2241, t2307, t10276, t73, t2244);
        let t39103 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1094(t2250);
        let (t39108, t39109) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1095(t16, t39031);
        let t39110 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1096(t25, t28, t39109, zeta_threshold);
        let t39130 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1097(t11152, t76, t2244, t2250, t2251, t2252, t2255, t2283, t2284, t2291, t2298, t2304, t39096, t39097, t39103, t39110, t608, t609, t629, t634, t638, t642, t66, t72, t80, t9258, t9263, t9268, t9312, t9313, t9321, t9324, t9330, t9333, t9339);
        let (t39177, t39213) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1098(t41, t42, t53, t54, t1028, t36, t9576, t2244, t2250, t2262, t2267, t2268, t2271, t2274, t39, t39097, t39103, t39110, t43, t44, t51, t55, t615, t618, t9258, t9277, t9287, t9289, t9292, t9293, t9296, t9300, t9304, sigma0);
        let t39217 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1099(t1864, t2244, t2245, t2250, t2283, t2304, t31, t33, t39103, t39110, t39213, t607, t628, t642, t65, t6509, t67, t80, t9247, t9248, t9251, t9258, t9259, t9260);
        let t39221 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1100(t2235, t2240, t2241, t2307, t39030, t39032, t39034, t39036, t39038, t39040, t39043, t39046, t39049, t39054, t39063, t39064, t39070, t39130, t39217, t605, t645, t86, t9228, t9231, t9239, t9240, t9243, t9342);
        let (t39223, t39231, t39235, t39246, t39249) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1101(t5, t39221, t112, t2363, t111, t9346, t2405, t2420, t702);
    (t39097, t39103, t39108, t39109, t39110, t39177, t39223, t39231, t39235, t39246, t39249)
}
