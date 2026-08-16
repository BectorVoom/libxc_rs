//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta377<F: Float>(t4166: F, t9666: F, t9973: F, t10024: F, t1500: F, t9670: F, t9600: F, t1540: F, t9698: F) -> (F, F, F, F, F, F) {
        let (t46881, t46957, t47047, t47092, t47275, t47787) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1179::<F>(t4166, t9666, t9973, t10024, t1500, t9670, t9600, t1540, t9698);
    (t46881, t46957, t47047, t47092, t47275, t47787)
}
