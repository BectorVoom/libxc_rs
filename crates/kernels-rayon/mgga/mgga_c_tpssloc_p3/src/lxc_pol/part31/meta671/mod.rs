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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta671(t26016: f64, t92047: f64, t2031: f64, t96425: f64, t23967: f64, t27972: f64, t27976: f64, t2032: f64, t23963: f64, t23970: f64, t26009: f64, t26954: f64, t83717: f64, t90098: f64, t90114: f64, t91954: f64, t92057: f64, t96422: f64, t96443: f64, t96473: f64, t96535: f64, t27982: f64, t7032: f64, t26959: f64, t7435: f64, t7432: f64, t91957: f64, t27966: f64, t23975: f64, t26055: f64, t26090: f64, t26911: f64, t27961: f64, t7026: f64, t7782: f64, t84190: f64, t96403: f64, t96502: f64, t96506: f64, t1409: f64, t605: f64, t63: f64, t84219: f64, t26063: f64, t26070: f64, t26073: f64, t26076: f64, t26945: f64, t7035: f64, t91907: f64, t96553: f64, t96556: f64, t96559: f64, t96562: f64, t55921: f64, t7025: f64, t2240: f64, t5392: f64, t26067: f64, t28935: f64, t6492: f64, t6495: f64, t91959: f64, t96393: f64, t96406: f64, t96479: f64, t96482: f64, t96517: f64, t96521: f64, t84242: f64, t84248: f64, t84280: f64, t91961: f64, t91980: f64, t91996: f64, t92001: f64, t92003: f64, t92008: f64, t92012: f64, t92031: f64, t92034: f64, t27937: f64, t1860: f64, t26028: f64, t27979: f64, t6486: f64, t7428: f64, t84285: f64, t92049: f64, t92056: f64, t96379: f64, t96383: f64, t96646: f64, t5: f64, t102145: f64, t102171: f64, t112: f64, t19450: f64, t19577: f64, t19596: f64, t1983: f64, t19994: f64, t20098: f64, t20109: f64, t2040: f64, t2075: f64, t2079: f64, t22574: f64, t23938: f64, t24432: f64, t24987: f64, t24995: f64, t26898: f64, t26977: f64, t27144: f64, t27145: f64, t28821: f64, t29222: f64, t33899: f64, t510: f64, t5161: f64, t5460: f64, t6876: f64, t7042: f64, t7170: f64, t7171: f64, t7217: f64, t74032: f64, t75203: f64, t75560: f64, t7685: f64, t7904: f64, t9016: f64, t96824: f64, t1307: f64, t2094: f64, t671: f64, t7786: f64, t100990: f64, t1266: f64, t1459: f64, t19289: f64, t20127: f64, t2036: f64, t24990: f64, t26905: f64, t26969: f64, t27188: f64, t28826: f64, t28959: f64, t29252: f64, t4026: f64, t4037: f64, t5361: f64, t5450: f64, t6287: f64, t652: f64, t7040: f64, t7156: f64, t75214: f64, t7890: f64, t7900: f64, t7943: f64, t84733: f64, t96356: f64, t97789: f64, t111: f64, t28942: f64, t5456: f64, t7039: f64, t1268: f64, t12725: f64, t1458: f64, t19451: f64, t19456: f64, t2039: f64, t27170: f64, t28002: f64, t4028: f64, t7056: f64, t7801: f64, t92090: f64, t96683: f64, t96709: f64, t19534: f64, t2314: f64, t26114: f64, t26117: f64, t28007: f64, t28951: f64, t33234: f64, t4072: f64, t5113: f64, t5493: f64, t55943: f64, t7676: f64, t96657: f64, t12020: f64, t7936: f64, t16022: f64, t20029: f64, t26224: f64, t5325: f64, t7214: f64, t7937: f64, t90493: f64, t90496: f64, t90498: f64, t90503: f64, t93306: f64, t93309: f64, t93310: f64, t93311: f64, t93333: f64, t96848: f64, t96851: f64, t96854: f64, t96857: f64, t96866: f64, t96868: f64, t96873: f64, t96878: f64, t1843: f64, t90551: f64, t90581: f64, t90582: f64, t93313: f64, t93359: f64, t93361: f64, t93362: f64, t96910: f64, t96920: f64, t96925: f64, t96929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t102198 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2001(t26016, t92047, t2031, t96425, t23967, t27972, t27976, t2032, t23963, t23970, t26009, t26954, t83717, t90098, t90114, t91954, t92057, t96422, t96443, t96473, t96535);
        let t102223 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2002(t27982, t7032, t26959, t7435, t7432, t91957, t27966, t23963, t23975, t26055, t26090, t26911, t27961, t27972, t27976, t7026, t7782, t84190, t96403, t96502, t96506);
        let t102252 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2003(t1409, t605, t63, t27961, t84219, t2032, t26063, t26070, t26073, t26076, t26911, t26945, t27982, t7035, t7432, t7435, t7782, t91907, t96553, t96556, t96559, t96562);
        let t102278 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2004(t55921, t7025, t2240, t5392, t63, t2032, t26067, t26911, t27966, t28935, t6492, t6495, t7026, t7035, t91959, t96393, t96406, t96479, t96482, t96517, t96521);
        let t102284 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2005(t84242, t84248, t84280, t91961, t91980, t91996, t92001, t92003, t92008, t92012, t92031, t92034);
        let t102305 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2006(t27937, t7032, t1860, t2031, t2032, t26028, t26945, t27979, t28935, t6486, t7035, t7428, t7782, t84285, t92049, t92056, t96379, t96383, t96646);
        let (t102310, t102320) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2007(t5, t102145, t102171, t102198, t102223, t102252, t102278, t102284, t102305, t112, t19450, t19577, t19596, t1983, t19994, t20098, t20109, t2040, t2075, t2079, t22574, t23938, t24432, t24987, t24995, t26898, t26977, t27144, t27145, t28821, t29222, t33899, t510, t5161, t5460, t6876, t7042, t7170, t7171, t7217, t74032, t75203, t75560, t7685, t7904, t9016, t96824);
        let (t102344, t102366) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2008(t1307, t2094, t671, t7786, t100990, t1266, t1459, t19289, t1983, t20127, t2036, t2040, t22574, t24432, t24987, t24990, t26905, t26969, t27188, t28826, t28959, t29252, t4026, t4037, t510, t5361, t5450, t6287, t652, t6876, t7040, t7042, t7156, t75214, t7685, t7890, t7900, t7943, t84733, t96356, t97789);
        let (t102386, t102401, t102403) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2009(t111, t28942, t5456, t7039, t100990, t102310, t1268, t12725, t1458, t19451, t19456, t2039, t27170, t28002, t4028, t671, t7056, t75560, t7801, t92090, t96356, t96683, t96709);
        let t102432 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2010(t102344, t1458, t19534, t2039, t2314, t23938, t26114, t26117, t26977, t27170, t27188, t28007, t28951, t33234, t4072, t5113, t5493, t55943, t7042, t7056, t7676, t7801, t96657);
        let t102475 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2011(t12020, t7936, t16022, t20029, t26224, t5325, t7214, t7937, t90493, t90496, t90498, t90503, t93306, t93309, t93310, t93311, t93333, t96848, t96851, t96854, t96857, t96866, t96868, t96873, t96878);
        let t102493 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2012(t1843, t90551, t90581, t90582, t93313, t93359, t93361, t93362, t96910, t96920, t96925, t96929);
    (t102320, t102366, t102386, t102401, t102403, t102432, t102475, t102493)
}
