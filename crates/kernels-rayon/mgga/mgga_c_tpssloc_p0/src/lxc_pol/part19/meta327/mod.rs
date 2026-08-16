//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1162;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1163;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1164;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1165;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1166;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1167;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta327(t12250: f64, t40045: f64, t550: f64, t1336: f64, t2690: f64, t3788: f64, t3795: f64, t3792: f64, t67: f64, t6924: f64, t246: f64, t12156: f64, t12012: f64, t120: f64, t12177: f64, t12371: f64, t16398: f64, t12283: f64, t12426: f64, t1307: f64, t3850: f64, t12291: f64, t12368: f64, t12397: f64, t12419: f64, t12420: f64, t1341: f64, t1343: f64, t1352: f64, t16233: f64, t16305: f64, t3790: f64, t3803: f64, t3805: f64, t3806: f64, t3807: f64, t3853: f64, t820: f64, t12392: f64, t3799: f64, t39249: f64, t39256: f64, t39261: f64, t39266: f64, t39304: f64, t39306: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39324: f64, t39327: f64, t39329: f64, t39331: f64, t39335: f64, t39338: f64, t39340: f64, t39342: f64, t39346: f64, t39349: f64, t39356: f64, t39360: f64, t39364: f64, t39366: f64, t39373: f64, t39375: f64, t39384: f64, t39388: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t39456: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39479: f64, t39483: f64, t39490: f64, t39492: f64, t39496: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39511: f64, t39513: f64, t39515: f64, t39518: f64, t39521: f64, t39523: f64, t39529: f64, t39531: f64, t39533: f64, t39539: f64, t39541: f64, t39549: f64, t39563: f64, t39570: f64, t39572: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40148, t40153, t40160, t40162, t40168, t40169) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1162(t12250, t40045, t550, t1336, t2690, t3788, t3795, t3792, t67, t6924, t246, t12156);
        let (t40183, t40197, t40204) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1163(t12012, t550, t120, t12177, t12371, t16398, t12283, t12426, t12250, t1307, t3850, t12291, t12368, t12397, t12419, t12420, t1341, t1343, t1352, t16233, t16305, t3790, t3803, t3805, t3806, t3807, t3853, t40148, t40153, t40160, t40162, t40168, t40169, t820);
        let (t40206, t40210) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1164(t12392, t3799, t39249, t39256, t39261, t39266, t39304, t39306, t39309, t39312, t39316, t39320, t39324, t39327);
        let t40211 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1165(t39329, t39331, t39335, t39338, t39340, t39342, t39346, t39349, t39356, t39360, t39364, t39366, t39373);
        let t40213 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1166(t39375, t39384, t39388, t39393, t39397, t39400, t39408, t39411, t39456, t39463, t39468, t39472);
        let t40214 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1167(t39476, t39479, t39483, t39490, t39492, t39496, t39499, t39502, t39505, t39508, t39511, t39513, t39515);
        let t40217 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1168(t39518, t39521, t39523, t39529, t39531, t39533, t39539, t39541, t39549, t39563, t39570, t39572);
    (t40148, t40153, t40162, t40183, t40197, t40204, t40206, t40210, t40211, t40213, t40214, t40217)
}
