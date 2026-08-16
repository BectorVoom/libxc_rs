//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2529;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2530;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2531;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta753(t18280: f64, t4869: f64, t51819: f64, t63519: f64, t71115: f64, t1117: f64, t11275: f64, t15067: f64, t6020: f64, t18271: f64, t18283: f64, t18915: f64, t4884: f64, t71225: f64, t71227: f64, t71230: f64, t71233: f64, t71236: f64, t18934: f64, t50834: f64, t51683: f64, t63291: f64, t63306: f64, t63308: f64, t63332: f64, t63334: f64, t63336: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71144: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t43816: f64, t43942: f64, t50919: f64, t51707: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t71183: f64, t71187: f64, t71191: f64, t71195: f64, t71199: f64, t71203: f64, t71206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71238, t71241, t71245, t71247, t71249, t71251) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2529(t18280, t4869, t51819, t63519, t71115, t1117, t11275, t15067, t6020, t18271, t18283, t18915, t4884);
        let (t71252, t71255) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2530(t71225, t71227, t71230, t71233, t71236, t71238, t71241, t71245, t71247, t71249, t71251, t18934, t4869);
        let t71289 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2531(t50834, t51683, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let t71308 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2532(t43816, t43942, t50919, t51707, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
    (t71238, t71241, t71245, t71247, t71249, t71251, t71252, t71255, t71289, t71308)
}
