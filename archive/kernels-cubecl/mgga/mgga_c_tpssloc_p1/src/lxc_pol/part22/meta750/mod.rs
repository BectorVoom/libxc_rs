//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2512;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2513;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2514;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2515;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2516;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2517;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta750<F: Float>(t21763: F, t690: F, t18205: F, t3966: F, t11145: F, t123: F, t50834: F, t51550: F, t63291: F, t63306: F, t63308: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t20234: F, t43763: F, t607: F, t43809: F, t5971: F, t1088: F, t21762: F, t20217: F, t3247: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t71156 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2512::<F>(t21763, t690);
        let (t71158, t71160) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2513::<F>(t18205, t3966, t11145, t123);
        let t71162 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2514::<F>(t50834, t51550, t63291, t63306, t63308, t63332, t63334, t63336, t71124, t71130, t71135, t71140, t71142, t71144, t71146, t71150, t71152, t71154, t71156, t71160);
        let (t71164, t71166) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2515::<F>(t20234, t43763, t607, t123, t43809);
        let (t71168, t71170) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2516::<F>(t3966, t5971, t1088, t123);
        let (t71172, t71174) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2517::<F>(t21762, t607, t1088, t123);
        let (t71177, t71179) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2518::<F>(t20217, t3247, t607, t1088, t123);
    (t71156, t71158, t71160, t71162, t71164, t71166, t71168, t71170, t71172, t71174, t71177, t71179)
}
