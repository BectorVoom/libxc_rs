//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2005;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta498<F: Float>(t12832: F, t16505: F, t3: F, t112: F, t5363: F, t111: F, t1851: F, t2319: F, t576: F, t4072: F, t671: F, t1458: F, t2363: F, t12521: F, t12524: F, t12813: F, t1401: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F) -> (F, F, F, F, F, F, F, F) {
        let (t16506, t16507, t16521, t16524, t16535, t16538, t16541) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2005::<F>(t12832, t16505, t3, t112, t5363, t111, t1851, t2319, t576, t4072, t671, t1458, t2363);
        let t16546 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2006::<F>(t12521, t12524, t12813, t1401, t1458, t16506, t16521, t16524, t16535, t16538, t16541, t2319, t2363, t3938, t3941, t4072, t5371, t5376, t577, t671);
    (t16506, t16507, t16521, t16524, t16535, t16538, t16541, t16546)
}
