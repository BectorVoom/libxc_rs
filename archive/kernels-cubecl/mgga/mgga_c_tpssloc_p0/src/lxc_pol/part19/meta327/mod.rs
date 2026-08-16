//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1162;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1163;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1164;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1165;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1166;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1167;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta327<F: Float>(t12250: F, t40045: F, t550: F, t1336: F, t2690: F, t3788: F, t3795: F, t3792: F, t67: F, t6924: F, t246: F, t12156: F, t12012: F, t120: F, t12177: F, t12371: F, t16398: F, t12283: F, t12426: F, t1307: F, t3850: F, t12291: F, t12368: F, t12397: F, t12419: F, t12420: F, t1341: F, t1343: F, t1352: F, t16233: F, t16305: F, t3790: F, t3803: F, t3805: F, t3806: F, t3807: F, t3853: F, t820: F, t12392: F, t3799: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39306: F, t39309: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39329: F, t39331: F, t39335: F, t39338: F, t39340: F, t39342: F, t39346: F, t39349: F, t39356: F, t39360: F, t39364: F, t39366: F, t39373: F, t39375: F, t39384: F, t39388: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t39456: F, t39463: F, t39468: F, t39472: F, t39476: F, t39479: F, t39483: F, t39490: F, t39492: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39511: F, t39513: F, t39515: F, t39518: F, t39521: F, t39523: F, t39529: F, t39531: F, t39533: F, t39539: F, t39541: F, t39549: F, t39563: F, t39570: F, t39572: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40148, t40153, t40160, t40162, t40168, t40169) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1162::<F>(t12250, t40045, t550, t1336, t2690, t3788, t3795, t3792, t67, t6924, t246, t12156);
        let (t40183, t40197, t40204) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1163::<F>(t12012, t550, t120, t12177, t12371, t16398, t12283, t12426, t12250, t1307, t3850, t12291, t12368, t12397, t12419, t12420, t1341, t1343, t1352, t16233, t16305, t3790, t3803, t3805, t3806, t3807, t3853, t40148, t40153, t40160, t40162, t40168, t40169, t820);
        let (t40206, t40210) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1164::<F>(t12392, t3799, t39249, t39256, t39261, t39266, t39304, t39306, t39309, t39312, t39316, t39320, t39324, t39327);
        let t40211 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1165::<F>(t39329, t39331, t39335, t39338, t39340, t39342, t39346, t39349, t39356, t39360, t39364, t39366, t39373);
        let t40213 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1166::<F>(t39375, t39384, t39388, t39393, t39397, t39400, t39408, t39411, t39456, t39463, t39468, t39472);
        let t40214 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1167::<F>(t39476, t39479, t39483, t39490, t39492, t39496, t39499, t39502, t39505, t39508, t39511, t39513, t39515);
        let t40217 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1168::<F>(t39518, t39521, t39523, t39529, t39531, t39533, t39539, t39541, t39549, t39563, t39570, t39572);
    (t40148, t40153, t40162, t40183, t40197, t40204, t40206, t40210, t40211, t40213, t40214, t40217)
}
