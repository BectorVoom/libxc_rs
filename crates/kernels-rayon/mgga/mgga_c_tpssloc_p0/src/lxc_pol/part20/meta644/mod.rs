//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta644 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2358;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2359;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2360;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2361;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2362;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2363;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2364;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2365;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2366;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta644(t13555: f64, t13784: f64, t2986: f64, t13528: f64, t1592: f64, t42891: f64, t973: f64, t13812: f64, t13822: f64, t13881: f64, t13886: f64, t10263: f64, t4506: f64, t10346: f64, t13813: f64, t13882: f64, t13887: f64, t1593: f64, t1597: f64, t2960: f64, t343: f64, t42554: f64, t4546: f64, t10186: f64, t10235: f64, t10325: f64, t13769: f64, t13798: f64, t13817: f64, t13874: f64, t13931: f64, t23494: f64, t42811: f64, t42817: f64, t42827: f64, t42830: f64, t42833: f64, t42839: f64, t42855: f64, t42858: f64, t42873: f64, t42877: f64, t42909: f64, t42911: f64, t42914: f64, t42916: f64, t42918: f64, t42925: f64, t42936: f64, t42944: f64, t42951: f64, t42962: f64, t42985: f64, t43055: f64, t43059: f64, t43075: f64, t4510: f64, t4511: f64, t4515: f64, t4518: f64, t4519: f64, t4523: f64, t4531: f64, t4549: f64, t45872: f64, t47689: f64, t47693: f64, t47720: f64, t47759: f64, t47763: f64, t47940: f64, t47941: f64, t47978: f64, t48017: f64, t48022: f64, t48024: f64, t48030: f64, t48044: f64, t48048: f64, t48076: f64, t48235: f64, t48242: f64, t48244: f64, t48250: f64, t48256: f64, t48260: f64, t48294: f64, t48297: f64, t48302: f64, t48317: f64, t48321: f64, t48361: f64, t48374: f64, t48379: f64, t48382: f64, t48384: f64, t48387: f64, t884: f64, t977: f64, t978: f64, t984: f64, t225: f64, t3082: f64, t4622: f64, t1040: f64, t13941: f64, t10231: f64, t1036: f64, t13751: f64, t10422: f64, t14229: f64, t3070: f64, t14234: f64, t42488: f64, t10390: f64, t10408: f64, t10413: f64, t10445: f64, t1046: f64, t13527: f64, t14218: f64, t14219: f64, t14228: f64, t14230: f64, t1611: f64, t2244: f64, t2250: f64, t2770: f64, t3071: f64, t360: f64, t369: f64, t378: f64, t42303: f64, t68: f64, t3121: f64, t607: f64, t1022: f64, t4649: f64, t41666: f64, t43398: f64, t1409: f64, t9288: f64, t14488: f64, t376: f64, t1023: f64, t1041: f64, t14107: f64, t14220: f64, t14222: f64, t3039: f64, t42322: f64, t42324: f64, t42354: f64, t42369: f64, t42372: f64, t42546: f64, t43211: f64, t4337: f64, t4342: f64, t4582: f64, t4588: f64, t45993: f64, t10214: f64, t10877: f64, t14130: f64, t14167: f64, t1539: f64, t2979: f64, t3048: f64, t42380: f64, t42403: f64, t42412: f64, t43361: f64, t4562: f64, t4565: f64, t47742: f64, t47767: f64, t14036: f64, t3966: f64, t13969: f64, t13976: f64, t3130: f64, t14183: f64, t10471: f64, t47840: f64, t10479: f64, t10908: f64, t4641: f64, t10485: f64, t10937: f64, t10965: f64, t14033: f64, t14037: f64, t14164: f64, t42428: f64, t42432: f64, t4585: f64, t4590: f64, t47697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48390, t48394, t48397, t48402, t48407, t48417, t48421) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2358(t13555, t13784, t2986, t13528, t1592, t42891, t973, t13812, t13822, t13881, t13886, t10263, t4506);
        let t48423 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2359(t10346, t13813, t13882, t13887, t1593, t1597, t2960, t343, t42554, t4546, t48394, t48397, t48402, t48407, t48417, t48421, t973);
        let t48427 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2360(t10186, t10235, t10263, t10325, t13769, t13798, t13817, t13874, t13931, t1597, t23494, t2960, t2986, t343, t42811, t42817, t42827, t42830, t42833, t42839, t42855, t42858, t42873, t42877, t42909, t42911, t42914, t42916, t42918, t42925, t42936, t42944, t42951, t42962, t42985, t43055, t43059, t43075, t4510, t4511, t4515, t4518, t4519, t4523, t4531, t4546, t4549, t45872, t47689, t47693, t47720, t47759, t47763, t47940, t47941, t47978, t48017, t48022, t48024, t48030, t48044, t48048, t48076, t48235, t48242, t48244, t48250, t48256, t48260, t48294, t48297, t48302, t48317, t48321, t48361, t48374, t48379, t48382, t48384, t48387, t48390, t48423, t884, t973, t977, t978, t984);
        let (t48428, t48431, t48432, t48441, t48446, t48460) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2361(t225, t48427, t3082, t4622, t1040, t13941, t10231, t13555, t973, t1036, t13751, t10422, t14229, t3070);
        let t48471 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2362(t14234, t3070, t42488, t10390, t10408, t10413, t10445, t1046, t13527, t14218, t14219, t14228, t14230, t1611, t2244, t2250, t2770, t3071, t360, t369, t378, t42303, t48428, t48431, t48432, t48441, t48446, t48460, t68);
        let (t48472, t48477, t48496, t48497) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2363(t3121, t607, t1022, t4649, t41666, t43398, t1409, t9288);
        let (t48506, t48511) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2364(t14488, t376, t1023, t10408, t1041, t10413, t14107, t14220, t14222, t3039, t3070, t3071, t42322, t42324, t42354, t42369, t42372, t42546, t43211, t4337, t4342, t4582, t4588, t45993, t48472, t48477, t48496, t48497);
        let t48543 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2365(t10214, t10263, t10390, t10877, t14130, t14167, t1539, t2979, t3048, t3071, t42380, t42403, t42412, t43361, t4562, t4565, t47689, t47693, t47720, t47742, t47767, t973, t977);
        let (t48548, t48554) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2366(t14036, t3070, t42488, t2244, t3966);
        let (t48569, t48577) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2367(t13969, t13976, t3130, t1041, t14183, t10471, t47840, t10479, t10908, t4641, t10485, t10937, t10965, t14033, t14037, t14164, t2979, t42428, t42432, t4582, t4585, t4590, t47697, t48548, t48554, t973);
    (t48427, t48428, t48471, t48497, t48506, t48511, t48543, t48554, t48569, t48577)
}
