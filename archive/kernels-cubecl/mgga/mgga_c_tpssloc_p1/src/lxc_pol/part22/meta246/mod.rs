//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1353;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1354;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta246<F: Float>(t10294: F, t268: F, t271: F, t6546: F, t2394: F, t885: F, t154: F, t3061: F, t276: F, t285: F, t273: F, t2928: F, t941: F, t2931: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10542, t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1353::<F>(t10294, t268, t271, t6546, t2394, t885);
        let (t10564, t10577, t10595, t10599, t10608, t10629) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1354::<F>(t154, t3061, t10544, t276, t285, t273, t2928, t941);
        let t10632 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1355::<F>(t2931, t323);
    (t10542, t10544, t10545, t10556, t10564, t10577, t10595, t10599, t10608, t10629, t10632)
}
