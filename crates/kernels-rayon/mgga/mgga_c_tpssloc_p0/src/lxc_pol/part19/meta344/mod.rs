//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1231;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta344(t41274: f64, t185: f64, t39110: f64, t707: f64, t2447: f64, t32: f64, t2659: f64, t9929: f64, t9932: f64, t31: f64, t717: f64, t9898: f64, t2658: f64, t39103: f64, t607: f64, t9862: f64, t2250: f64, t4194: f64, t750: f64, t39658: f64, t41266: f64, t41270: f64, t41273: f64, t6589: f64, t68: f64, t13151: f64, t1891: f64, t225: f64, t228: f64, t230: f64, t2379: f64, t2553: f64, t2667: f64, t2671: f64, t2672: f64, t2675: f64, t40848: f64, t40972: f64, t40977: f64, t41241: f64, t41242: f64, t41244: f64, t41245: f64, t41248: f64, t41249: f64, t41263: f64, t4225: f64, t822: f64, t824: f64, t825: f64, t9516: f64, t9938: f64, t9947: f64, t9950: f64, t9951: f64, t9954: f64, t232: f64, t2617: f64, t9670: f64, t831: f64, t13254: f64, t237: f64, t249: f64, t2618: f64, t2623: f64, t2645: f64, t41123: f64, t41130: f64, t41132: f64, t41134: f64, t41139: f64, t41231: f64, t41237: f64, t4178: f64, t817: f64, t819: f64, t820: f64, t9618: f64, t9626: f64, t9634: f64, t9663: f64, t9960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41275, t41278, t41281, t41283, t41286) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230(t41274, t185, t39110, t707, t2447, t32, t2659, t9929, t9932, t31, t717, t9898);
        let (t41289, t41292, t41296, t41297) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1231(t185, t2658, t39103, t607, t707, t9862, t2250, t4194, t750, t39658, t41266, t41270, t41273, t41275, t41278, t41281, t41283, t41286);
        let t41332 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232(t6589, t68, t13151, t1891, t225, t228, t230, t2379, t2553, t2667, t2671, t2672, t2675, t40848, t40972, t40977, t41241, t41242, t41244, t41245, t41248, t41249, t41263, t41297, t4225, t822, t824, t825, t9516, t9938, t9947, t9950, t9951, t9954);
        let (t41333, t41343) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233(t232, t41332, t2617, t9670, t831, t13254, t237, t249, t2618, t2623, t2645, t41123, t41130, t41132, t41134, t41139, t41231, t41237, t4178, t817, t819, t820, t9618, t9626, t9634, t9663, t9960);
    (t41275, t41278, t41281, t41283, t41286, t41289, t41292, t41296, t41333, t41343)
}
