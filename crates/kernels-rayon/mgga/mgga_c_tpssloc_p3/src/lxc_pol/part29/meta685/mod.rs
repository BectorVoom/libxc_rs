//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2338;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2339;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2340;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2341;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2342;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2343;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2344;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2345;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2346;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2347;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2348;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta685(t27331: f64, t9231: f64, t2110: f64, t22519: f64, t22531: f64, t22537: f64, t24511: f64, t24526: f64, t26090: f64, t27332: f64, t6492: f64, t7246: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64, t85514: f64, t85524: f64, t90297: f64, t90337: f64, t90340: f64, t24505: f64, t24508: f64, t26070: f64, t26073: f64, t26076: f64, t7256: f64, t7259: f64, t90150: f64, t90153: f64, t90160: f64, t90343: f64, t46104: f64, t7245: f64, t12571: f64, t24525: f64, t9239: f64, t22527: f64, t22546: f64, t24514: f64, t26055: f64, t27341: f64, t85510: f64, t90196: f64, t90202: f64, t90205: f64, t2240: f64, t27363: f64, t33: f64, t24520: f64, t26063: f64, t26067: f64, t27308: f64, t27311: f64, t27365: f64, t6495: f64, t90177: f64, t90227: f64, t90232: f64, t90334: f64, t26012: f64, t7255: f64, t22549: f64, t24517: f64, t26009: f64, t27298: f64, t27303: f64, t83722: f64, t83778: f64, t85463: f64, t85480: f64, t85501: f64, t85536: f64, t90080: f64, t90114: f64, t90137: f64, t90141: f64, t2109: f64, t90090: f64, t90094: f64, t45844: f64, t26016: f64, t85470: f64, t85473: f64, t85476: f64, t85507: f64, t90072: f64, t90076: f64, t90098: f64, t90101: f64, t90104: f64, t22550: f64, t7974: f64, t90247: f64, t1419: f64, t2274: f64, t12606: f64, t12648: f64, t12652: f64, t14165: f64, t1860: f64, t1864: f64, t2108: f64, t2244: f64, t2250: f64, t24498: f64, t26028: f64, t27356: f64, t27364: f64, t608: f64, t6486: f64, t6509: f64, t67: f64, t7251: f64, t7428: f64, t83803: f64, t85539: f64, t90121: f64, t22489: f64, t22493: f64, t22534: f64, t24504: f64, t26024: f64, t7445: f64, t90132: f64, t90257: f64, t5: f64, t112: f64, t671: f64, t7263: f64, t12813: f64, t1459: f64, t1849: f64, t2165: f64, t2314: f64, t24932: f64, t24939: f64, t27293: f64, t3929: f64, t4037: f64, t510: f64, t652: f64, t8107: f64, t91666: f64, t91671: f64, t91673: f64, t91674: f64, t91678: f64, t91681: f64, t91684: f64, t91690: f64, t91694: f64, t91698: f64, t91704: f64, t91706: f64, t2320: f64, t8103: f64, t91708: f64, t91713: f64, t91715: f64, t91718: f64, t91722: f64, t91724: f64, t91726: f64, t91730: f64, t91735: f64, t91737: f64, t91739: f64, t91747: f64, t91749: f64, t91752: f64, t91755: f64, t91757: f64, t91759: f64, t91762: f64, t111: f64, t27370: f64, t1458: f64, t2363: f64, t27863: f64, t27888: f64, t4072: f64, t7266: f64, t85428: f64, t90355: f64, t90361: f64, t90363: f64, t90365: f64, t90367: f64, t90369: f64, t94248: f64, t90372: f64, t90374: f64, t90377: f64, t90379: f64, t90383: f64, t90385: f64, t90387: f64, t90399: f64, t90404: f64, t90406: f64, t90408: f64, t90410: f64, t94265: f64) -> (f64, f64, f64, f64, f64) {
        let t95996 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2338(t27331, t9231, t2110, t22519, t22531, t22537, t24511, t24526, t26090, t27332, t6492, t7246, t7432, t7435, t7975, t7978, t85514, t85524, t90297, t90337, t90340);
        let t96021 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2339(t2110, t24505, t24508, t26070, t26073, t26076, t7256, t7259, t7435, t90150, t90153, t90160, t90343);
        let t96050 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2340(t46104, t7245, t12571, t24525, t27331, t9239, t2110, t22527, t22531, t22537, t22546, t24514, t26055, t27341, t6492, t7256, t7259, t7432, t7978, t85510, t90196, t90202, t90205);
        let t96077 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2341(t2240, t27363, t33, t24520, t24526, t26063, t26067, t27308, t27311, t27365, t6492, t6495, t7246, t90177, t90227, t90232, t90334);
        let t96105 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2342(t26012, t7255, t22527, t22549, t24514, t24517, t24520, t26009, t26090, t27298, t27303, t27332, t6495, t83722, t83778, t85463, t85480, t85501, t85536, t90080, t90114, t90137, t90141);
        let t96133 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2343(t2109, t90090, t90094, t45844, t7245, t22546, t22549, t24514, t24517, t26016, t7432, t85470, t85473, t85476, t85507, t90072, t90076, t90098, t90101, t90104);
        let t96180 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2344(t22550, t7974, t2109, t90247, t1419, t2274, t12606, t12648, t12652, t14165, t1860, t1864, t2108, t2110, t2244, t2250, t22549, t24498, t24505, t24508, t26009, t26028, t27303, t27356, t27364, t27365, t608, t6486, t6509, t67, t7251, t7256, t7259, t7428, t83803, t85539, t90121, t9239);
        let t96209 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2345(t1860, t2109, t2110, t22489, t22493, t22534, t24504, t24511, t26024, t27308, t27311, t6486, t7255, t7428, t7445, t7974, t7975, t7978, t90132, t90257);
        let (t96214, t96222, t96228) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2346(t5, t95996, t96021, t96050, t96077, t96105, t96133, t96180, t96209, t112, t671, t7263, t12813, t1459, t1849, t2165, t2314, t24932, t24939, t27293, t3929, t4037, t510, t652, t8107, t91666, t91671, t91673, t91674, t91678, t91681, t91684, t91690, t91694, t91698, t91704, t91706);
        let t96232 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2347(t2320, t8103, t91708, t91713, t91715, t91718, t91722, t91724, t91726, t91730, t91735, t91737, t91739, t91747, t91749, t91752, t91755, t91757, t91759, t91762);
        let (t96238, t96269) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2348(t111, t27370, t12813, t1458, t2363, t24932, t27863, t27888, t4072, t671, t7266, t85428, t90355, t90361, t90363, t90365, t90367, t90369, t94248, t96222);
        let t96271 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2349(t90372, t90374, t90377, t90379, t90383, t90385, t90387, t90399, t90404, t90406, t90408, t90410, t94265, t96214);
    (t96228, t96232, t96238, t96269, t96271)
}
