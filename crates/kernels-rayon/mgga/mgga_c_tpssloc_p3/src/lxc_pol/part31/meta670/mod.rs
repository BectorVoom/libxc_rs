//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta670 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1989;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1990;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1991;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1992;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1993;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1994;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1995;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1996;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1997;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1998;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1999;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta670(t101832: f64, t870: f64, t193: f64, t7859: f64, t16557: f64, t1877: f64, t2057: f64, t24191: f64, t24339: f64, t25: f64, t25024: f64, t2522: f64, t25375: f64, t25377: f64, t25385: f64, t26563: f64, t26744: f64, t28256: f64, t28459: f64, t29106: f64, t4314: f64, t606: f64, t7110: f64, t7114: f64, t7845: f64, t97950: f64, t97953: f64, t97985: f64, t98015: f64, t98034: f64, t98075: f64, t100562: f64, t16662: f64, t16944: f64, t16949: f64, t17109: f64, t24344: f64, t25365: f64, t25374: f64, t4119: f64, t5527: f64, t5544: f64, t5664: f64, t67128: f64, t67164: f64, t84800: f64, t93000: f64, t98007: f64, t98011: f64, t98030: f64, t100572: f64, t101226: f64, t1484: f64, t1530: f64, t16596: f64, t202: f64, t26740: f64, t28248: f64, t29125: f64, t4255: f64, t4303: f64, t46341: f64, t5660: f64, t67123: f64, t776: f64, t84766: f64, t868: f64, t92276: f64, t97999: f64, t98003: f64, t98102: f64, t265: f64, t394: f64, t101209: f64, t101248: f64, t101283: f64, t1409: f64, t16558: f64, t2064: f64, t26807: f64, t29149: f64, t3966: f64, t40: f64, t5398: f64, t607: f64, t7131: f64, t7865: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t100638: f64, t100641: f64, t100644: f64, t100656: f64, t100669: f64, t100696: f64, t100731: f64, t100769: f64, t100791: f64, t1081: f64, t25928: f64, t25930: f64, t26756: f64, t28: f64, t28771: f64, t84797: f64, t100651: f64, t100682: f64, t100692: f64, t100713: f64, t100743: f64, t101196: f64, t101211: f64, t101220: f64, t101241: f64, t1649: f64, t25892: f64, t25905: f64, t25921: f64, t28774: f64, t28778: f64, t7649: f64, t92319: f64, t100664: f64, t100705: f64, t100708: f64, t100766: f64, t100788: f64, t18196: f64, t25898: f64, t25901: f64, t25938: f64, t28795: f64, t29157: f64, t6841: f64, t6848: f64, t7656: f64, t100646: f64, t100659: f64, t100689: f64, t100718: f64, t100734: f64, t100747: f64, t100759: f64, t100772: f64, t100780: f64, t25934: f64, t25945: f64, t28764: f64, t28789: f64, t28792: f64, t5966: f64, t504: f64, t2071: f64, t26862: f64, t29189: f64, t52: f64, t7150: f64, t7884: f64, rho1: f64, t101138: f64, t101150: f64, t113: f64, t1442: f64, t15868: f64, t19451: f64, t1983: f64, t22574: f64, t24175: f64, t26161: f64, t26163: f64, t26558: f64, t26559: f64, t26870: f64, t26902: f64, t26906: f64, t26974: f64, t28821: f64, t28834: f64, t28969: f64, t29197: f64, t29377: f64, t29378: f64, t5107: f64, t650: f64, t6876: f64, t6879: f64, t6999: f64, t7050: f64, t7218: f64, t7685: f64, t7787: f64, t7940: f64, t91655: f64, t92169: f64, t96797: f64, t97875: f64, t97894: f64, t26959: f64, t7428: f64, t27979: f64, t7032: f64, t1860: f64, t27956: f64, t7031: f64, t91890: f64, t91894: f64, t91896: f64, t91898: f64, t91900: f64, t91904: f64, t91905: f64, t91913: f64, t91921: f64, t2031: f64, t96461: f64, t96469: f64, t22549: f64, t23963: f64, t26009: f64, t26016: f64, t26954: f64, t34125: f64, t84216: f64, t84229: f64, t90101: f64, t90104: f64, t91922: f64, t92040: f64, t92052: f64, t9239: f64, t96418: f64, t96458: f64, t96466: f64) -> (f64, f64, f64) {
        let (t101833, t101840, t101843) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1989(t101832, t870, t193, t7859, t16557, t1877, t2057, t24191, t24339, t25, t25024, t2522, t25375, t25377, t25385, t26563, t26744, t28256, t28459, t29106, t4314, t606, t7110, t7114, t7845, t97950, t97953, t97985, t98015, t98034, t98075);
        let t101892 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1990(t100562, t16662, t16944, t16949, t17109, t1877, t2057, t24344, t2522, t25365, t25374, t26563, t26744, t4119, t4314, t5527, t5544, t5664, t67128, t67164, t7110, t7114, t7845, t84800, t93000, t98007, t98011, t98030);
        let t101937 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1991(t100572, t101226, t101832, t1484, t1530, t16596, t1877, t193, t202, t24191, t24339, t24344, t2522, t26740, t26744, t28248, t29106, t29125, t4255, t4303, t4314, t46341, t5660, t67123, t7114, t776, t7845, t84766, t868, t870, t92276, t97999, t98003, t98102);
        let (t101938, t101951) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1992(t25, t265, t394, t101892, t101937, t101209, t101248, t101283, t101843, t1409, t16558, t2064, t26807, t29149, t3966, t40, t5398, t607, t7131, t7865, dens_threshold, rho0, zeta_threshold);
        let t101981 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1993(t100638, t100641, t100644, t100656, t100669, t100696, t100731, t100769, t100791, t101833, t101840, t1081, t1877, t24191, t24344, t25928, t25930, t26563, t26744, t26756, t28, t28771, t29106, t7114, t84797);
        let t102012 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1994(t100651, t100682, t100692, t100713, t100743, t101196, t101211, t101220, t101241, t1649, t1877, t2057, t24191, t2522, t25892, t25905, t25921, t26563, t26740, t26756, t28774, t28778, t7110, t7649, t7845, t92319);
        let t102048 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1995(t100664, t100705, t100708, t100766, t100788, t101226, t18196, t1877, t2057, t24191, t24339, t2522, t25898, t25901, t25938, t26563, t28795, t29106, t29157, t46341, t6841, t6848, t7656, t7845, t92276, t92319);
        let t102087 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1996(t100646, t100659, t100689, t100718, t100734, t100747, t100759, t100772, t100780, t1877, t2057, t24191, t24339, t2522, t25934, t25945, t26744, t26756, t28764, t28789, t28792, t4314, t5966, t7110, t7114, t84800);
        let t102102 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1997(t28, t265, t504, t101938, t101981, t102012, t102048, t102087, t1409, t16558, t2071, t26862, t29189, t3966, t52, t5398, t607, t7150, t7884, dens_threshold, rho1, zeta_threshold);
        let t102105 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1998(t101138, t101150, t101951, t102102, t113, t1442, t15868, t19451, t1983, t22574, t24175, t26161, t26163, t26558, t26559, t26870, t26902, t26906, t26974, t28821, t28834, t28969, t29197, t29377, t29378, t5107, t650, t6876, t6879, t6999, t7050, t7218, t7685, t7787, t7940, t91655, t92169, t96797, t97875, t97894);
        let t102145 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1999(t26959, t7428, t27979, t7032, t1860, t27956, t7031, t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91913, t91921);
        let t102171 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2000(t2031, t96461, t96469, t22549, t23963, t26009, t26016, t26954, t34125, t84216, t84229, t90101, t90104, t91922, t92040, t92052, t9239, t96418, t96458, t96466);
    (t102105, t102145, t102171)
}
