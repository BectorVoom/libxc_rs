//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta417<F: Float>(t1118: F, t14913: F, t1099: F, t14720: F, t14722: F, t14704: F, t11136: F, t11137: F, t11139: F, t11141: F, t11143: F, t14702: F, t14708: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> (F, F, F, F, F, F) {
        let (t14914, t14916, t14922, t14923, t14924, t14933) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1935::<F>(t1118, t14913, t1099, t14720, t14722, t14704, t11136, t11137, t11139, t11141, t11143, t14702, t14708, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
    (t14914, t14916, t14922, t14923, t14924, t14933)
}
