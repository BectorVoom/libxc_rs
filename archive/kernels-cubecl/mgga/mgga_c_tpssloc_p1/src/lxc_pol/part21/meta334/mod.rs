//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta334<F: Float>(t9218: F, t9220: F, t12560: F, t12561: F, t12562: F, t12563: F, t9225: F, t3951: F, t604: F, t1406: F, t2239: F, t1437: F, t2241: F) -> (F, F, F, F, F, F) {
        let (t12564, t12565, t12566, t12568, t12571, t12582) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1711::<F>(t9218, t9220, t12560, t12561, t12562, t12563, t9225, t3951, t604, t1406, t2239, t1437, t2241);
    (t12564, t12565, t12566, t12568, t12571, t12582)
}
