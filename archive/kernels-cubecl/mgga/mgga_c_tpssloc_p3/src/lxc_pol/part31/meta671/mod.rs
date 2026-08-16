//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta671 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2001;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2002;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2003;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2004;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2005;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2006;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2007;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2008;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2009;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2010;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2011;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta671<F: Float>(t26016: F, t92047: F, t2031: F, t96425: F, t23967: F, t27972: F, t27976: F, t2032: F, t23963: F, t23970: F, t26009: F, t26954: F, t83717: F, t90098: F, t90114: F, t91954: F, t92057: F, t96422: F, t96443: F, t96473: F, t96535: F, t27982: F, t7032: F, t26959: F, t7435: F, t7432: F, t91957: F, t27966: F, t23975: F, t26055: F, t26090: F, t26911: F, t27961: F, t7026: F, t7782: F, t84190: F, t96403: F, t96502: F, t96506: F, t1409: F, t605: F, t63: F, t84219: F, t26063: F, t26070: F, t26073: F, t26076: F, t26945: F, t7035: F, t91907: F, t96553: F, t96556: F, t96559: F, t96562: F, t55921: F, t7025: F, t2240: F, t5392: F, t26067: F, t28935: F, t6492: F, t6495: F, t91959: F, t96393: F, t96406: F, t96479: F, t96482: F, t96517: F, t96521: F, t84242: F, t84248: F, t84280: F, t91961: F, t91980: F, t91996: F, t92001: F, t92003: F, t92008: F, t92012: F, t92031: F, t92034: F, t27937: F, t1860: F, t26028: F, t27979: F, t6486: F, t7428: F, t84285: F, t92049: F, t92056: F, t96379: F, t96383: F, t96646: F, t5: F, t102145: F, t102171: F, t112: F, t19450: F, t19577: F, t19596: F, t1983: F, t19994: F, t20098: F, t20109: F, t2040: F, t2075: F, t2079: F, t22574: F, t23938: F, t24432: F, t24987: F, t24995: F, t26898: F, t26977: F, t27144: F, t27145: F, t28821: F, t29222: F, t33899: F, t510: F, t5161: F, t5460: F, t6876: F, t7042: F, t7170: F, t7171: F, t7217: F, t74032: F, t75203: F, t75560: F, t7685: F, t7904: F, t9016: F, t96824: F, t1307: F, t2094: F, t671: F, t7786: F, t100990: F, t1266: F, t1459: F, t19289: F, t20127: F, t2036: F, t24990: F, t26905: F, t26969: F, t27188: F, t28826: F, t28959: F, t29252: F, t4026: F, t4037: F, t5361: F, t5450: F, t6287: F, t652: F, t7040: F, t7156: F, t75214: F, t7890: F, t7900: F, t7943: F, t84733: F, t96356: F, t97789: F, t111: F, t28942: F, t5456: F, t7039: F, t1268: F, t12725: F, t1458: F, t19451: F, t19456: F, t2039: F, t27170: F, t28002: F, t4028: F, t7056: F, t7801: F, t92090: F, t96683: F, t96709: F, t19534: F, t2314: F, t26114: F, t26117: F, t28007: F, t28951: F, t33234: F, t4072: F, t5113: F, t5493: F, t55943: F, t7676: F, t96657: F, t12020: F, t7936: F, t16022: F, t20029: F, t26224: F, t5325: F, t7214: F, t7937: F, t90493: F, t90496: F, t90498: F, t90503: F, t93306: F, t93309: F, t93310: F, t93311: F, t93333: F, t96848: F, t96851: F, t96854: F, t96857: F, t96866: F, t96868: F, t96873: F, t96878: F, t1843: F, t90551: F, t90581: F, t90582: F, t93313: F, t93359: F, t93361: F, t93362: F, t96910: F, t96920: F, t96925: F, t96929: F) -> (F, F, F, F, F, F, F, F) {
        let t102198 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2001::<F>(t26016, t92047, t2031, t96425, t23967, t27972, t27976, t2032, t23963, t23970, t26009, t26954, t83717, t90098, t90114, t91954, t92057, t96422, t96443, t96473, t96535);
        let t102223 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2002::<F>(t27982, t7032, t26959, t7435, t7432, t91957, t27966, t23963, t23975, t26055, t26090, t26911, t27961, t27972, t27976, t7026, t7782, t84190, t96403, t96502, t96506);
        let t102252 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2003::<F>(t1409, t605, t63, t27961, t84219, t2032, t26063, t26070, t26073, t26076, t26911, t26945, t27982, t7035, t7432, t7435, t7782, t91907, t96553, t96556, t96559, t96562);
        let t102278 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2004::<F>(t55921, t7025, t2240, t5392, t63, t2032, t26067, t26911, t27966, t28935, t6492, t6495, t7026, t7035, t91959, t96393, t96406, t96479, t96482, t96517, t96521);
        let t102284 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2005::<F>(t84242, t84248, t84280, t91961, t91980, t91996, t92001, t92003, t92008, t92012, t92031, t92034);
        let t102305 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2006::<F>(t27937, t7032, t1860, t2031, t2032, t26028, t26945, t27979, t28935, t6486, t7035, t7428, t7782, t84285, t92049, t92056, t96379, t96383, t96646);
        let (t102310, t102320) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2007::<F>(t5, t102145, t102171, t102198, t102223, t102252, t102278, t102284, t102305, t112, t19450, t19577, t19596, t1983, t19994, t20098, t20109, t2040, t2075, t2079, t22574, t23938, t24432, t24987, t24995, t26898, t26977, t27144, t27145, t28821, t29222, t33899, t510, t5161, t5460, t6876, t7042, t7170, t7171, t7217, t74032, t75203, t75560, t7685, t7904, t9016, t96824);
        let (t102344, t102366) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2008::<F>(t1307, t2094, t671, t7786, t100990, t1266, t1459, t19289, t1983, t20127, t2036, t2040, t22574, t24432, t24987, t24990, t26905, t26969, t27188, t28826, t28959, t29252, t4026, t4037, t510, t5361, t5450, t6287, t652, t6876, t7040, t7042, t7156, t75214, t7685, t7890, t7900, t7943, t84733, t96356, t97789);
        let (t102386, t102401, t102403) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2009::<F>(t111, t28942, t5456, t7039, t100990, t102310, t1268, t12725, t1458, t19451, t19456, t2039, t27170, t28002, t4028, t671, t7056, t75560, t7801, t92090, t96356, t96683, t96709);
        let t102432 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2010::<F>(t102344, t1458, t19534, t2039, t2314, t23938, t26114, t26117, t26977, t27170, t27188, t28007, t28951, t33234, t4072, t5113, t5493, t55943, t7042, t7056, t7676, t7801, t96657);
        let t102475 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2011::<F>(t12020, t7936, t16022, t20029, t26224, t5325, t7214, t7937, t90493, t90496, t90498, t90503, t93306, t93309, t93310, t93311, t93333, t96848, t96851, t96854, t96857, t96866, t96868, t96873, t96878);
        let t102493 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2012::<F>(t1843, t90551, t90581, t90582, t93313, t93359, t93361, t93362, t96910, t96920, t96925, t96929);
    (t102320, t102366, t102386, t102401, t102403, t102432, t102475, t102493)
}
