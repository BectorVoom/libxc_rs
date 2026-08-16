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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta644<F: Float>(t13555: F, t13784: F, t2986: F, t13528: F, t1592: F, t42891: F, t973: F, t13812: F, t13822: F, t13881: F, t13886: F, t10263: F, t4506: F, t10346: F, t13813: F, t13882: F, t13887: F, t1593: F, t1597: F, t2960: F, t343: F, t42554: F, t4546: F, t10186: F, t10235: F, t10325: F, t13769: F, t13798: F, t13817: F, t13874: F, t13931: F, t23494: F, t42811: F, t42817: F, t42827: F, t42830: F, t42833: F, t42839: F, t42855: F, t42858: F, t42873: F, t42877: F, t42909: F, t42911: F, t42914: F, t42916: F, t42918: F, t42925: F, t42936: F, t42944: F, t42951: F, t42962: F, t42985: F, t43055: F, t43059: F, t43075: F, t4510: F, t4511: F, t4515: F, t4518: F, t4519: F, t4523: F, t4531: F, t4549: F, t45872: F, t47689: F, t47693: F, t47720: F, t47759: F, t47763: F, t47940: F, t47941: F, t47978: F, t48017: F, t48022: F, t48024: F, t48030: F, t48044: F, t48048: F, t48076: F, t48235: F, t48242: F, t48244: F, t48250: F, t48256: F, t48260: F, t48294: F, t48297: F, t48302: F, t48317: F, t48321: F, t48361: F, t48374: F, t48379: F, t48382: F, t48384: F, t48387: F, t884: F, t977: F, t978: F, t984: F, t225: F, t3082: F, t4622: F, t1040: F, t13941: F, t10231: F, t1036: F, t13751: F, t10422: F, t14229: F, t3070: F, t14234: F, t42488: F, t10390: F, t10408: F, t10413: F, t10445: F, t1046: F, t13527: F, t14218: F, t14219: F, t14228: F, t14230: F, t1611: F, t2244: F, t2250: F, t2770: F, t3071: F, t360: F, t369: F, t378: F, t42303: F, t68: F, t3121: F, t607: F, t1022: F, t4649: F, t41666: F, t43398: F, t1409: F, t9288: F, t14488: F, t376: F, t1023: F, t1041: F, t14107: F, t14220: F, t14222: F, t3039: F, t42322: F, t42324: F, t42354: F, t42369: F, t42372: F, t42546: F, t43211: F, t4337: F, t4342: F, t4582: F, t4588: F, t45993: F, t10214: F, t10877: F, t14130: F, t14167: F, t1539: F, t2979: F, t3048: F, t42380: F, t42403: F, t42412: F, t43361: F, t4562: F, t4565: F, t47742: F, t47767: F, t14036: F, t3966: F, t13969: F, t13976: F, t3130: F, t14183: F, t10471: F, t47840: F, t10479: F, t10908: F, t4641: F, t10485: F, t10937: F, t10965: F, t14033: F, t14037: F, t14164: F, t42428: F, t42432: F, t4585: F, t4590: F, t47697: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48390, t48394, t48397, t48402, t48407, t48417, t48421) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2358::<F>(t13555, t13784, t2986, t13528, t1592, t42891, t973, t13812, t13822, t13881, t13886, t10263, t4506);
        let t48423 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2359::<F>(t10346, t13813, t13882, t13887, t1593, t1597, t2960, t343, t42554, t4546, t48394, t48397, t48402, t48407, t48417, t48421, t973);
        let t48427 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2360::<F>(t10186, t10235, t10263, t10325, t13769, t13798, t13817, t13874, t13931, t1597, t23494, t2960, t2986, t343, t42811, t42817, t42827, t42830, t42833, t42839, t42855, t42858, t42873, t42877, t42909, t42911, t42914, t42916, t42918, t42925, t42936, t42944, t42951, t42962, t42985, t43055, t43059, t43075, t4510, t4511, t4515, t4518, t4519, t4523, t4531, t4546, t4549, t45872, t47689, t47693, t47720, t47759, t47763, t47940, t47941, t47978, t48017, t48022, t48024, t48030, t48044, t48048, t48076, t48235, t48242, t48244, t48250, t48256, t48260, t48294, t48297, t48302, t48317, t48321, t48361, t48374, t48379, t48382, t48384, t48387, t48390, t48423, t884, t973, t977, t978, t984);
        let (t48428, t48431, t48432, t48441, t48446, t48460) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2361::<F>(t225, t48427, t3082, t4622, t1040, t13941, t10231, t13555, t973, t1036, t13751, t10422, t14229, t3070);
        let t48471 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2362::<F>(t14234, t3070, t42488, t10390, t10408, t10413, t10445, t1046, t13527, t14218, t14219, t14228, t14230, t1611, t2244, t2250, t2770, t3071, t360, t369, t378, t42303, t48428, t48431, t48432, t48441, t48446, t48460, t68);
        let (t48472, t48477, t48496, t48497) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2363::<F>(t3121, t607, t1022, t4649, t41666, t43398, t1409, t9288);
        let (t48506, t48511) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2364::<F>(t14488, t376, t1023, t10408, t1041, t10413, t14107, t14220, t14222, t3039, t3070, t3071, t42322, t42324, t42354, t42369, t42372, t42546, t43211, t4337, t4342, t4582, t4588, t45993, t48472, t48477, t48496, t48497);
        let t48543 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2365::<F>(t10214, t10263, t10390, t10877, t14130, t14167, t1539, t2979, t3048, t3071, t42380, t42403, t42412, t43361, t4562, t4565, t47689, t47693, t47720, t47742, t47767, t973, t977);
        let (t48548, t48554) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2366::<F>(t14036, t3070, t42488, t2244, t3966);
        let (t48569, t48577) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2367::<F>(t13969, t13976, t3130, t1041, t14183, t10471, t47840, t10479, t10908, t4641, t10485, t10937, t10965, t14033, t14037, t14164, t2979, t42428, t42432, t4582, t4585, t4590, t47697, t48548, t48554, t973);
    (t48427, t48428, t48471, t48497, t48506, t48511, t48543, t48554, t48569, t48577)
}
