//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2466;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta665<F: Float>(t3242: F, t405: F, t974: F, t1176: F, t2402: F, t1174: F, t1179: F, t11529: F, t3460: F, t3456: F, t10469: F, t1190: F) -> (F, F, F, F, F, F, F) {
        let (t44620, t44621, t44633, t44635, t44638, t44641, t44690) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2466::<F>(t3242, t405, t974, t1176, t2402, t1174, t1179, t11529, t3460, t3456, t10469, t1190);
    (t44620, t44621, t44633, t44635, t44638, t44641, t44690)
}
