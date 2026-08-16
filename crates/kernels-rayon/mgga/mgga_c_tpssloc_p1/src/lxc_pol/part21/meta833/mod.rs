//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta833 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2941;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2942;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2943;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2944;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2945;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2946;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2947;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2948;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2949;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2950;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2951;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta833(t13784: f64, t17178: f64, t2986: f64, t10189: f64, t5836: f64, t2990: f64, t17161: f64, t17152: f64, t48213: f64, t17863: f64, t42837: f64, t10186: f64, t17808: f64, t10255: f64, t17800: f64, t17804: f64, t42830: f64, t42962: f64, t42968: f64, t4510: f64, t5821: f64, t59715: f64, t10236: f64, t17635: f64, t13835: f64, t13847: f64, t13839: f64, t48279: f64, t17748: f64, t10235: f64, t13851: f64, t4531: f64, t48021: f64, t48024: f64, t48030: f64, t48044: f64, t48048: f64, t48052: f64, t48357: f64, t17849: f64, t2960: f64, t5838: f64, t698: f64, t973: f64, t5844: f64, t4540: f64, t4509: f64, t10190: f64, t17794: f64, t10237: f64, t340: f64, t343: f64, t4518: f64, t48061: f64, t48063: f64, t48066: f64, t48068: f64, t48189: f64, t59730: f64, t974: f64, t10263: f64, t13769: f64, t13831: f64, t4347: f64, t48207: f64, t48210: f64, t48215: f64, t48233: f64, t48242: f64, t48244: f64, t48250: f64, t48256: f64, t5839: f64, t59767: f64, t6733: f64, t5842: f64, t17686: f64, t42841: f64, t17783: f64, t13779: f64, t17167: f64, t10241: f64, t10245: f64, t17817: f64, t42846: f64, t48281: f64, t5818: f64, t5825: f64, t59659: f64, t17171: f64, t17157: f64, t17769: f64, t10224: f64, t5824: f64, t13822: f64, t17752: f64, t17753: f64, t17758: f64, t17778: f64, t3008: f64, t4546: f64, t59755: f64, t59763: f64, t17757: f64, t17772: f64, t2970: f64, t13931: f64, t17773: f64, t17841: f64, t48292: f64, t48297: f64, t48302: f64, t48317: f64, t48320: f64, t48328: f64, t55677: f64, t7577: f64, t977: f64, t978: f64, t984: f64, t17777: f64, t13798: f64, t1597: f64, t17857: f64, t17860: f64, t17864: f64, t2978: f64, t2994: f64, t48336: f64, t48338: f64, t48342: f64, t55723: f64, t5829: f64, t59751: f64, t61065: f64, t4514: f64, t48019: f64, t48046: f64, t10259: f64, t17742: f64, t17745: f64, t17749: f64, t17801: f64, t25608: f64, t3014: f64, t59719: f64, t59746: f64, t884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61245, t61252, t61258, t61261, t61264, t61273) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2941(t13784, t17178, t2986, t10189, t5836, t2990, t17161, t17152, t48213, t17863, t42837, t10186, t17808);
        let t61275 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2942(t10255, t17800, t17804, t2986, t42830, t42962, t42968, t4510, t5821, t59715, t61245, t61252, t61258, t61261, t61264, t61273);
        let t61301 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2943(t10236, t17635, t13835, t13847, t2986, t13839, t48279, t17748, t10235, t13851, t4531, t48021, t48024, t48030, t48044, t48048, t48052, t48357);
        let (t61307, t61310, t61313, t61315, t61322, t61327) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2944(t17849, t2960, t5838, t698, t973, t5844, t4540, t4509, t5836, t10190, t17794, t2986);
        let t61332 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2945(t10237, t2986, t340, t343, t4518, t48061, t48063, t48066, t48068, t48189, t59730, t61307, t61310, t61313, t61315, t61322, t61327, t973, t974);
        let t61355 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2946(t10263, t13769, t13831, t17800, t2986, t4347, t4518, t4531, t48207, t48210, t48215, t48233, t48242, t48244, t48250, t48256, t5839, t59767, t6733);
        let t61389 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2947(t4509, t5842, t17686, t42841, t17783, t2960, t13779, t17167, t2986, t10235, t10237, t10241, t10245, t10263, t17804, t17817, t17863, t42846, t4518, t48281, t5818, t5825, t59659);
        let (t61391, t61394, t61397, t61405, t61408) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2948(t13779, t17171, t2986, t13784, t17157, t10190, t17817, t17769, t2960, t10224, t5824, t973);
        let t61424 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2949(t13822, t17752, t973, t17753, t17758, t17778, t2960, t2986, t3008, t343, t4510, t4518, t4546, t5842, t59755, t59763, t61391, t61394, t61397, t61405, t61408);
        let t61453 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2950(t13822, t17757, t973, t17772, t2970, t13931, t17773, t17841, t2960, t343, t4546, t48292, t48297, t48302, t48317, t48320, t48328, t55677, t7577, t977, t978, t984);
        let t61485 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2951(t13822, t17777, t973, t10186, t10263, t13798, t13839, t1597, t17857, t17860, t17864, t2978, t2986, t2994, t3008, t343, t4546, t48336, t48338, t48342, t55723, t5829, t5836, t59751, t61065, t977, t984);
        let t61523 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2952(t2986, t4514, t48019, t48046, t10186, t10259, t17742, t17745, t17749, t17794, t17801, t17817, t25608, t3014, t343, t4510, t4518, t4531, t4546, t5836, t59719, t59746, t884, t973);
    (t61275, t61301, t61332, t61355, t61389, t61424, t61453, t61485, t61523)
}
