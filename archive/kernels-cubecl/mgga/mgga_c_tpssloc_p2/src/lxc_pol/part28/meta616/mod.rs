//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta616<F: Float>(t26301: F, t80853: F, t80855: F, t22788: F, t5314: F, t16333: F, t6952: F, t1831: F, t80866: F, t131: F, t6931: F, t9537: F, t26322: F, t236: F, t26318: F, t91005: F, t22782: F, t5234: F, t1369: F, t26257: F, t3876: F, t80849: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91143, t91145, t91147, t91149, t91152) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1932::<F>(t26301, t80853, t80855, t22788, t5314, t16333, t6952, t1831, t80866, t131, t6931, t9537);
        let (t91154, t91158, t91161, t91163, t91165) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1933::<F>(t26322, t80855, t91152, t236, t26318, t91005, t22782, t5234, t1369, t26257, t3876, t1831, t80849);
    (t91143, t91145, t91147, t91149, t91154, t91158, t91161, t91163, t91165)
}
