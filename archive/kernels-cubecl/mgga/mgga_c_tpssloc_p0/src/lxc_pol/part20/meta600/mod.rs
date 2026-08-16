//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta600<F: Float>(t11153: F, t460: F, t9288: F, t3242: F, t405: F, t974: F, t11509: F, t1174: F, t15281: F, t11525: F, t3431: F, t1176: F, t2402: F) -> (F, F, F, F, F, F, F) {
        let (t44607, t44608, t44620, t44621, t44628, t44631, t44633) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2180::<F>(t11153, t460, t9288, t3242, t405, t974, t11509, t1174, t15281, t11525, t3431, t1176, t2402);
    (t44607, t44608, t44620, t44621, t44628, t44631, t44633)
}
