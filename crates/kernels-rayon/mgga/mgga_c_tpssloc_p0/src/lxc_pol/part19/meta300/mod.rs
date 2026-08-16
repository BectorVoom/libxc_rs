//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1084;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta300(t1229: f64, t3242: f64, t3493: f64, t3508: f64, t11153: f64, t3584: f64, t1089: f64, t1215: f64, t607: f64, t475: f64, t1332: f64, t5343: f64, t12248: f64, t68: f64, t544: f64, t5333: f64, t5194: f64, t782: f64, t3732: f64, t67: f64, t792: f64, t12214: f64, t131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15615, t15620, t15654, t15661, t15708, t16033) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1084(t1229, t3242, t3493, t3508, t11153, t3584, t1089, t1215, t607, t475, t1332, t5343);
        let (t16047, t16055, t16081, t16094, t16100) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1085(t12248, t68, t544, t1332, t5333, t5194, t782, t3732, t67, t792, t12214, t131);
    (t15615, t15620, t15654, t15661, t15708, t16033, t16047, t16055, t16081, t16094, t16100)
}
