//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta753 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2529;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2530;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2531;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta753<F: Float>(t18280: F, t4869: F, t51819: F, t63519: F, t71115: F, t1117: F, t11275: F, t15067: F, t6020: F, t18271: F, t18283: F, t18915: F, t4884: F, t71225: F, t71227: F, t71230: F, t71233: F, t71236: F, t18934: F, t50834: F, t51683: F, t63291: F, t63306: F, t63308: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t43816: F, t43942: F, t50919: F, t51707: F, t63361: F, t63382: F, t63384: F, t63398: F, t63400: F, t71166: F, t71170: F, t71174: F, t71179: F, t71183: F, t71187: F, t71191: F, t71195: F, t71199: F, t71203: F, t71206: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t71238, t71241, t71245, t71247, t71249, t71251) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2529::<F>(t18280, t4869, t51819, t63519, t71115, t1117, t11275, t15067, t6020, t18271, t18283, t18915, t4884);
        let (t71252, t71255) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2530::<F>(t71225, t71227, t71230, t71233, t71236, t71238, t71241, t71245, t71247, t71249, t71251, t18934, t4869);
        let t71289 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2531::<F>(t50834, t51683, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let t71308 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2532::<F>(t43816, t43942, t50919, t51707, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
    (t71238, t71241, t71245, t71247, t71249, t71251, t71252, t71255, t71289, t71308)
}
