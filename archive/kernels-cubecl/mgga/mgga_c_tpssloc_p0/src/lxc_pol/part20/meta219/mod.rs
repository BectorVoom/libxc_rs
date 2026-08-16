//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1287;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta219<F: Float>(t3: F, t5363: F, t112: F, t1851: F, t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t577: F, t154: F, t781: F, t202: F, t243: F, t2229: F, t61: F, t119: F, t212: F, t252: F, t828: F, t343: F, t984: F, t3034: F, t334: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5364, t5371, t5376, t5381, t6546) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1287::<F>(t3, t5363, t112, t1851, t1458, t671, t1401, t3938, t3941, t4072, t577, t154, t781);
        let (t6589, t6597, t6600, t6647, t6733, t6739) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1288::<F>(t202, t243, t2229, t61, t119, t212, t252, t828, t343, t984, t3034, t334);
    (t5364, t5371, t5376, t5381, t6546, t6589, t6597, t6600, t6647, t6733, t6739)
}
