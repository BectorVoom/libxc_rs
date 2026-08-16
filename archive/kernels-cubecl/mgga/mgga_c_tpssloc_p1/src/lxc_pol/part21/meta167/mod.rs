//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1084;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta167<F: Float>(t31: F, t3966: F, t65: F, t1410: F, t628: F, t1426: F, t608: F, t1409: F, t2267: F, t607: F, t43: F, t2274: F, t55: F, t1414: F, t1420: F, t2282: F, t39: F, t51: F, t615: F, t621: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3967, t3968, t3971, t3976, t3981, t3982, t3985, t3990) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1084::<F>(t31, t3966, t65, t1410, t628, t1426, t608, t1409, t2267, t607, t43, t2274);
        let (t3991, t3994, t3997) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1085::<F>(t3990, t607, t3966, t55, t1414, t1420, t2282, t39, t3982, t3985, t51, t615, t621);
    (t3967, t3968, t3971, t3976, t3981, t3982, t3985, t3990, t3991, t3994, t3997)
}
