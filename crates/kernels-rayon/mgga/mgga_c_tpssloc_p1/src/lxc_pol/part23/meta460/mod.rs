//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1346;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta460(t42110: f64, t42113: f64, t76637: f64, t959: f64, t17934: f64, t5804: f64, t5694: f64, t42100: f64, t42102: f64, t5695: f64, t60357: f64, t21268: f64, t49489: f64, t10702: f64, t2844: f64, t1557: f64, t68924: f64, t17195: f64, t5727: f64, t5730: f64, t59959: f64, t21300: f64, t4354: f64, t1637: f64, t4700: f64, t68711: f64, t76634: f64, t76636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76641, t76643, t76644, t76647, t76652, t76654) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1346(t42110, t42113, t76637, t959, t17934, t5804, t5694, t42100, t42102, t5695, t60357, t21268, t49489);
        let (t76657, t76659, t76661, t76663, t76665, t76666) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1347(t10702, t2844, t76644, t1557, t68924, t17195, t5727, t5730, t59959, t21300, t4354, t1637, t4700, t68711, t76634, t76636, t76641, t76643, t76647, t76652, t76654);
    (t76641, t76643, t76644, t76647, t76652, t76654, t76657, t76659, t76661, t76663, t76665, t76666)
}
