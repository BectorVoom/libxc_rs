//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1084;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta300<F: Float>(t1229: F, t3242: F, t3493: F, t3508: F, t11153: F, t3584: F, t1089: F, t1215: F, t607: F, t475: F, t1332: F, t5343: F, t12248: F, t68: F, t544: F, t5333: F, t5194: F, t782: F, t3732: F, t67: F, t792: F, t12214: F, t131: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15615, t15620, t15654, t15661, t15708, t16033) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1084::<F>(t1229, t3242, t3493, t3508, t11153, t3584, t1089, t1215, t607, t475, t1332, t5343);
        let (t16047, t16055, t16081, t16094, t16100) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1085::<F>(t12248, t68, t544, t1332, t5333, t5194, t782, t3732, t67, t792, t12214, t131);
    (t15615, t15620, t15654, t15661, t15708, t16033, t16047, t16055, t16081, t16094, t16100)
}
