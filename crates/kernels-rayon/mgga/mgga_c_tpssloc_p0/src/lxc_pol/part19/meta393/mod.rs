//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1491;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1492;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1493;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1494;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1495;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta393(t626: f64, t9412: f64, t106: f64, t9364: f64, t2332: f64, t2358: f64, t2248: f64, t35761: f64, t2350: f64, t2354: f64, t39108: f64, t35577: f64, t2342: f64, t100: f64, t103: f64, t2336: f64, t2341: f64, t2343: f64, t2346: f64, t2349: f64, t657: f64, t660: f64, t92: f64, t9276: f64, t9374: f64, t9384: f64, t9386: f64, t9389: f64, t9390: f64, t9393: f64, t9394: f64, t9398: f64, t9403: f64, t9407: f64, t95: f64, t96: f64, tau0: f64, t2331: f64, t45421: f64, t45422: f64, t45424: f64, t45426: f64, t45428: f64, t45430: f64, t64: f64, t656: f64, t9365: f64, t9370: f64, t9411: f64, t109: f64, t11968: f64, t11972: f64, t12504: f64, t12507: f64, t1266: f64, t1268: f64, t12734: f64, t2312: f64, t2314: f64, t2323: f64, t2363: f64, t2364: f64, t3652: f64, t39223: f64, t39231: f64, t39235: f64, t4034: f64, t45408: f64, t510: f64, t5113: f64, t574: f64, t650: f64, t652: f64, t671: f64, t88: f64, t9348: f64, t9416: f64, t45405: f64, t112: f64, t12512: f64, t111: f64, t3931: f64, t12521: f64, t12524: f64, t12529: f64, t12532: f64, t1395: f64, t1401: f64, t16535: f64, t2319: f64, t3938: f64, t3941: f64, t577: f64, t12513: f64, t12537: f64, t1396: f64, t1398: f64, t1404: f64, t3: f64, t39022: f64, t39024: f64, t39026: f64, t39028: f64, t3932: f64, t3946: f64, t580: f64) -> f64 {
        let (t45432, t45435, t45436, t45444, t45453, t45460, t45461, t45469, t45482, t45496) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1491(t626, t9412, t106, t9364, t2332, t2358, t2248, t35761, t2350, t2354, t39108, t35577);
        let t45505 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1492(t2342, t100, t103, t2248, t2336, t2341, t2343, t2346, t2349, t2350, t2354, t45453, t45460, t45461, t45469, t45482, t45496, t657, t660, t92, t9276, t9374, t9384, t9386, t9389, t9390, t9393, t9394, t9398, t9403, t9407, t95, t96, tau0);
        let t45509 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1493(t2331, t2332, t2358, t45421, t45422, t45424, t45426, t45428, t45430, t45432, t45435, t45436, t45444, t45505, t64, t656, t9365, t9370, t9411);
        let (t45510, t45545) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1494(t109, t45509, t11968, t11972, t12504, t12507, t1266, t1268, t12734, t2312, t2314, t2323, t2363, t2364, t3652, t39223, t39231, t39235, t4034, t45408, t510, t5113, t574, t650, t652, t671, t88, t9348, t9416);
        let (t45546, t45580) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1495(t45405, t45545, t112, t12512, t111, t3931, t12521, t12524, t12529, t12532, t1395, t1401, t16535, t2319, t2363, t39231, t3938, t3941, t45510, t577, t671, t9416);
        let tv4rho40 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1496(t12513, t12537, t1396, t1398, t1404, t3, t39022, t39024, t39026, t39028, t3932, t3946, t45546, t45580, t580);
    tv4rho40
}
