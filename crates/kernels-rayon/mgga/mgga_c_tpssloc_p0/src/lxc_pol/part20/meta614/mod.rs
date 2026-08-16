//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta614 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2204;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2205;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2206;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2207;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2208;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2209;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2210;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2211;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2212;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2213;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2214;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta614(t1454: f64, t2585: f64, t2281: f64, t4044: f64, t12758: f64, t626: f64, t12761: f64, t12754: f64, t4068: f64, t12809: f64, t92: f64, t9384: f64, t100: f64, t9398: f64, t2341: f64, t657: f64, t12774: f64, t12775: f64, t12778: f64, t12795: f64, t1447: f64, t2219: f64, t2248: f64, t2336: f64, t2342: f64, t2350: f64, t2354: f64, t30171: f64, t30307: f64, t4049: f64, t4050: f64, t4054: f64, t659: f64, t662: f64, t9212: f64, t9393: f64, t9404: f64, t4063: f64, t591: f64, t4053: f64, t103: f64, t12771: f64, t12781: f64, t12784: f64, t1444: f64, t1445: f64, t1449: f64, t16: f64, t2349: f64, t4059: f64, t45460: f64, t45496: f64, t584: f64, t9374: f64, t9385: f64, t9399: f64, t9400: f64, t9407: f64, t9408: f64, t95: f64, t12757: f64, t12808: f64, t1453: f64, t2331: f64, t2358: f64, t26129: f64, t29903: f64, t45424: f64, t45428: f64, t45430: f64, t45435: f64, t64: f64, t656: f64, t666: f64, t9366: f64, t109: f64, t2332: f64, t4043: f64, t4067: f64, t45421: f64, t45422: f64, t45426: f64, t45432: f64, t9365: f64, t9411: f64, t25: f64, t28: f64, t88: f64, t9416: f64, t1406: f64, t9238: f64, t39031: f64, zeta_threshold: f64, t10913: f64, t12595: f64, t12598: f64, t12606: f64, t12609: f64, t12612: f64, t1409: f64, t2244: f64, t2250: f64, t2291: f64, t2298: f64, t39096: f64, t39114: f64, t3966: f64, t4007: f64, t4012: f64, t607: f64, t634: f64, t638: f64, t9258: f64, t9288: f64, t9321: f64, t9330: f64, t12677: f64, t12681: f64, t12684: f64, t12687: f64, t1414: f64, t1420: f64, t2262: f64, t39: f64, t39210: f64, t3982: f64, t3985: f64, t43: f64, t51: f64, t55: f64, t615: f64, t9277: f64, t9301: f64, t9308: f64, t9287: f64, t3961: f64, t9300: f64, t12680: f64, t12698: f64, t2267: f64, t2274: f64, t39159: f64, t39168: f64, t3981: f64, t3990: f64, t9305: f64, t12620: f64, t12630: f64, t1427: f64, t1434: f64, t2245: f64, t2284: f64, t2304: f64, t33: f64, t3997: f64, t3998: f64, t4018: f64, t629: f64, t642: f64, t66: f64, t72: f64, t80: f64, t9251: f64, t9313: f64, t9339: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t45656, t45659, t45660, t45662, t45676, t45689, t45690, t45697) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2204(t1454, t2585, t2281, t4044, t12758, t626, t12761, t12754, t4068, t12809, t92, t9384);
        let t45731 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2205(t100, t9398, t2341, t657, t12774, t12775, t12778, t12795, t1447, t2219, t2248, t2336, t2342, t2350, t2354, t30171, t30307, t4049, t4050, t4054, t45697, t659, t662, t92, t9212, t9393, t9404);
        let t45775 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2206(t100, t4063, t591, t4053, t92, t103, t12771, t12781, t12784, t1444, t1445, t1447, t1449, t16, t2341, t2349, t4059, t45460, t45496, t584, t657, t659, t662, t9374, t9385, t9399, t9400, t9407, t9408, t95);
        let t45780 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2207(t12757, t12808, t1453, t2331, t2358, t26129, t29903, t45424, t45428, t45430, t45435, t45676, t45689, t45690, t45731, t45775, t64, t656, t666, t9366);
        let t45782 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2208(t109, t2332, t4043, t4067, t45421, t45422, t45426, t45432, t45656, t45659, t45660, t45662, t45780, t64, t9365, t9411);
        let (t45814, t45844, t45872) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2209(t25, t28, t88, t9416, t1406, t9238, t16, t39031, zeta_threshold);
        let t45892 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2210(t10913, t12595, t12598, t12606, t12609, t12612, t1409, t2244, t2250, t2291, t2298, t39096, t39114, t3966, t4007, t4012, t45872, t607, t634, t638, t9258, t9288, t9321, t9330);
        let t45931 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2211(t12677, t12681, t12684, t12687, t1414, t1420, t2262, t39, t39210, t3982, t3985, t43, t45872, t51, t55, t615, t9277, t9301, t9308);
        let (t45970, t45971) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2212(t39, t9287, t2250, t3961);
        let t45977 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2213(t51, t9300, t12606, t12680, t12698, t1409, t1420, t2244, t2250, t2267, t2274, t39, t39159, t39168, t3966, t3981, t3990, t45970, t45971, t607, t9258, t9287, t9288, t9305);
        let t45986 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2214(t12620, t12630, t1427, t1434, t2244, t2245, t2284, t2304, t33, t3997, t3998, t4018, t45892, t45931, t45977, t629, t642, t66, t72, t80, t9251, t9313, t9339);
        let t45993 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2215(t1409, t9258);
    (t45782, t45814, t45844, t45872, t45971, t45986, t45993)
}
