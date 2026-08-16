//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta597<F: Float>(t11496: F, t3448: F, t11502: F, t1184: F, t15418: F, t11571: F, t3447: F, t3469: F, t4899: F, t11570: F, t9288: F, t3450: F, t9258: F) -> (F, F, F, F, F, F, F) {
        let (t44517, t44521, t44525, t44527, t44529, t44536, t44540) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2177::<F>(t11496, t3448, t11502, t1184, t15418, t11571, t3447, t3469, t4899, t11570, t9288, t3450, t9258);
    (t44517, t44521, t44525, t44527, t44529, t44536, t44540)
}
