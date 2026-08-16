//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta580<F: Float>(t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t10471: F, t44690: F, t11727: F, t44722: F, t478: F, t11718: F, t11147: F, t3439: F) -> (F, F, F, F, F, F, F, F) {
        let (t44833, t44834, t44836, t44857, t44858, t44863, t44896, t44938) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2089::<F>(t42341, t44696, t42344, t483, t1210, t10471, t44690, t11727, t44722, t478, t11718, t11147, t3439);
    (t44833, t44834, t44836, t44857, t44858, t44863, t44896, t44938)
}
