//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta484 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1472;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1473;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1474;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1475;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1476;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1477;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1478;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1479;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1480;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1481;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1482;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta484<F: Float>(t225: F, t78637: F, t11546: F, t1174: F, t15569: F, t15740: F, t1653: F, t1726: F, t22162: F, t22244: F, t22280: F, t22288: F, t3440: F, t3577: F, t3578: F, t45112: F, t484: F, t488: F, t52628: F, t52879: F, t53274: F, t66500: F, t68: F, t73043: F, t73113: F, t78035: F, t78039: F, t1177: F, t1196: F, t1227: F, t1735: F, t18321: F, t21758: F, t22129: F, t22133: F, t22137: F, t22197: F, t22258: F, t3560: F, t45128: F, t4582: F, t4889: F, t4987: F, t5024: F, t6184: F, t6188: F, t73076: F, t75847: F, t75912: F, t77621: F, t78043: F, t78047: F, t974: F, t11678: F, t1214: F, t19083: F, t21776: F, t22012: F, t22185: F, t22309: F, t248: F, t44725: F, t44863: F, t45250: F, t53238: F, t53440: F, t5979: F, t6203: F, t6225: F, t66545: F, t73084: F, t73096: F, t73099: F, t73102: F, t79018: F, t22119: F, t22154: F, t3555: F, t44805: F, t44817: F, t44938: F, t53490: F, t5975: F, t6178: F, t6192: F, t6219: F, t65884: F, t66622: F, t66668: F, t73142: F, t75836: F, t78689: F, t78713: F, t78734: F, t78775: F, t79024: F, t79056: F, t79087: F, t79120: F, t79160: F, t79188: F, t79214: F, t79251: F, t6243: F, t1751: F, t22298: F, t491: F, t78757: F, t6224: F, t6238: F, t11914: F, t11915: F, t1244: F, t1246: F, t15245: F, t1734: F, t1755: F, t1756: F, t19201: F, t22243: F, t22327: F, t22354: F, t22355: F, t22389: F, t3610: F, t3612: F, t3624: F, t3625: F, t6218: F, t6252: F, t6253: F, t6257: F, t65254: F, t73630: F, t11881: F, t11883: F, t11888: F, t15027: F, t1729: F, t22349: F, t22358: F, t22368: F, t22369: F, t22375: F, t22387: F, t3508: F, t44753: F, t44754: F, t44785: F, t44786: F, t470: F, t493: F, t5064: F, t53592: F, t53613: F, t6256: F, t6260: F, t6739: F, t11606: F, t11889: F, t1238: F, t1241: F, t1720: F, t1758: F, t1761: F, t19232: F, t19249: F, t22008: F, t22114: F, t22341: F, t22361: F, t22365: F, t22372: F, t22386: F, t22390: F, t22394: F, t44698: F, t44701: F, t44724: F, t44726: F, t45350: F, t466: F, t494: F, t4945: F, t498: F, t5055: F, t53565: F, t6168: F, t6244: F, t6261: F, t6263: F, t6265: F, t6267: F, t6268: F, t65262: F, t73613: F, t73856: F, t73891: F, t79008: F, t1256: F, t1763: F, t193: F, t336: F, t43706: F, t4700: F, t71101: F, t78344: F, t78348: F, t78355: F, t78357: F, t78359: F, t78361: F, t78364: F, t78367: F, t78370: F, t78373: F, t78646: F, t79005: F, t28: F, t265: F, t504: F, t76559: F, t78240: F, t78305: F, t78342: F, t1409: F, t1534: F, t1649: F, t1768: F, t20217: F, t20390: F, t21076: F, t22414: F, t506: F, t52: F, t5398: F, t5669: F, t5966: F, t6279: F, t77953: F, dens_threshold: F, rho1: F, zeta_threshold: F, t5389: F, t5445: F, t1411: F, t1426: F, t1427: F, t1434: F, t19420: F, t19430: F, t20210: F, t20218: F, t20219: F, t20285: F, t2291: F, t2298: F, t31: F, t39096: F, t39114: F, t4007: F, t4012: F, t5392: F, t5393: F, t5403: F, t5427: F, t5428: F, t5442: F, t634: F, t638: F, t65: F, t66: F, t72: F, t80: F, t1420: F, t1423: F, t19368: F, t19390: F, t20246: F, t20255: F, t20258: F, t20261: F, t2267: F, t2274: F, t39: F, t39159: F, t39168: F, t39210: F, t3981: F, t3990: F, t43: F, t51: F, t5416: F, t5421: F, t5424: F, t55: F, t56: F, t78505: F) -> (F, F, F, F, F) {
        let (t79260, t79282) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1472::<F>(t225, t78637, t11546, t1174, t15569, t15740, t1653, t1726, t22162, t22244, t22280, t22288, t3440, t3577, t3578, t45112, t484, t488, t52628, t52879, t53274, t66500, t68, t73043, t73113, t78035, t78039);
        let t79320 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1473::<F>(t1174, t1177, t1196, t1227, t1735, t18321, t21758, t22129, t22133, t22137, t22197, t22258, t3560, t3577, t45128, t4582, t4889, t4987, t5024, t6184, t6188, t73076, t75847, t75912, t77621, t78043, t78047, t974);
        let t79349 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1474::<F>(t11678, t1214, t1735, t19083, t21776, t22012, t22185, t22309, t248, t3577, t3578, t44725, t44863, t45250, t4889, t5024, t53238, t53440, t5979, t6203, t6225, t66545, t73084, t73096, t73099, t73102, t79018);
        let t79387 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1475::<F>(t1174, t15569, t18321, t22119, t22154, t3555, t3577, t3578, t44805, t44817, t44938, t4889, t53490, t5975, t5979, t6178, t6192, t6219, t65884, t66622, t66668, t73142, t75836, t75847, t974);
        let t79391 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1476::<F>(t78689, t78713, t78734, t78775, t79024, t79056, t79087, t79120, t79160, t79188, t79214, t79251, t79282, t79320, t79349, t79387);
        let (t79398, t79410, t79453, t79461, t79467) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1477::<F>(t6243, t1751, t22298, t491, t78757, t6224, t6238, t11914, t11915, t1244, t1246, t15245, t1734, t1755, t1756, t19201, t22243, t22327, t22354, t22355, t22389, t3610, t3612, t3624, t3625, t6218, t6252, t6253, t6257, t65254, t73630);
        let (t79473, t79524) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1478::<F>(t491, t79018, t11881, t11883, t11888, t15027, t1729, t22349, t22358, t22368, t22369, t22375, t22387, t3508, t3610, t44753, t44754, t44785, t44786, t470, t493, t5064, t53592, t53613, t6224, t6256, t6260, t6739, t79391, t79410);
        let t79533 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1479::<F>(t11606, t11881, t11883, t11888, t11889, t1238, t1241, t1244, t1246, t15027, t15245, t1720, t1751, t1758, t1761, t19201, t19232, t19249, t22008, t22114, t22243, t22327, t22341, t22354, t22361, t22365, t22372, t22386, t22390, t22394, t3610, t3612, t3624, t44698, t44701, t44724, t44726, t45350, t466, t491, t494, t4945, t498, t5055, t5064, t53565, t6168, t6218, t6238, t6243, t6244, t6252, t6261, t6263, t6265, t6267, t6268, t65262, t73613, t73856, t73891, t79008, t79260, t79391, t79398, t79410, t79453, t79461, t79467, t79473, t79524);
        let t79538 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1480::<F>(t1256, t1763, t193, t336, t43706, t4700, t71101, t78344, t78348, t78355, t78357, t78359, t78361, t78364, t78367, t78370, t78373, t78646, t79005, t79533);
        let t79553 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1481::<F>(t28, t265, t504, t76559, t78240, t78305, t78342, t79538, t1409, t1534, t1649, t1768, t20217, t20390, t21076, t22414, t506, t52, t5398, t5669, t5966, t6279, t75912, t77953, dens_threshold, rho1, zeta_threshold);
        let (t79579, t79585, t79637) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1482::<F>(t5389, t5445, t1411, t1426, t1427, t1434, t19420, t19430, t20210, t20217, t20218, t20219, t20285, t2291, t2298, t31, t39096, t39114, t4007, t4012, t5392, t5393, t5398, t5403, t5427, t5428, t5442, t634, t638, t65, t66, t72, t75836, t75847, t75912, t80);
        let t79692 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1483::<F>(t1420, t1423, t19368, t19390, t20217, t20246, t20255, t20258, t20261, t2267, t2274, t39, t39159, t39168, t39210, t3981, t3990, t43, t51, t5398, t5416, t5421, t5424, t55, t56, t75836, t75847, t75912, t78505);
    (t79553, t79579, t79585, t79637, t79692)
}
