//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2512;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2513;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2514;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2515;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2516;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2517;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta750(t21763: f64, t690: f64, t18205: f64, t3966: f64, t11145: f64, t123: f64, t50834: f64, t51550: f64, t63291: f64, t63306: f64, t63308: f64, t63332: f64, t63334: f64, t63336: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71144: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t20234: f64, t43763: f64, t607: f64, t43809: f64, t5971: f64, t1088: f64, t21762: f64, t20217: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t71156 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2512(t21763, t690);
        let (t71158, t71160) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2513(t18205, t3966, t11145, t123);
        let t71162 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2514(t50834, t51550, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let (t71164, t71166) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2515(t20234, t43763, t607, t123, t43809);
        let (t71168, t71170) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2516(t3966, t5971, t1088, t123);
        let (t71172, t71174) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2517(t21762, t607, t1088, t123);
        let (t71177, t71179) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2518(t20217, t3247, t607, t1088, t123);
    (t71156, t71158, t71160, t71162, t71164, t71166, t71168, t71170, t71172, t71174, t71177, t71179)
}
