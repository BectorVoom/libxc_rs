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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta484(t225: f64, t78637: f64, t11546: f64, t1174: f64, t15569: f64, t15740: f64, t1653: f64, t1726: f64, t22162: f64, t22244: f64, t22280: f64, t22288: f64, t3440: f64, t3577: f64, t3578: f64, t45112: f64, t484: f64, t488: f64, t52628: f64, t52879: f64, t53274: f64, t66500: f64, t68: f64, t73043: f64, t73113: f64, t78035: f64, t78039: f64, t1177: f64, t1196: f64, t1227: f64, t1735: f64, t18321: f64, t21758: f64, t22129: f64, t22133: f64, t22137: f64, t22197: f64, t22258: f64, t3560: f64, t45128: f64, t4582: f64, t4889: f64, t4987: f64, t5024: f64, t6184: f64, t6188: f64, t73076: f64, t75847: f64, t75912: f64, t77621: f64, t78043: f64, t78047: f64, t974: f64, t11678: f64, t1214: f64, t19083: f64, t21776: f64, t22012: f64, t22185: f64, t22309: f64, t248: f64, t44725: f64, t44863: f64, t45250: f64, t53238: f64, t53440: f64, t5979: f64, t6203: f64, t6225: f64, t66545: f64, t73084: f64, t73096: f64, t73099: f64, t73102: f64, t79018: f64, t22119: f64, t22154: f64, t3555: f64, t44805: f64, t44817: f64, t44938: f64, t53490: f64, t5975: f64, t6178: f64, t6192: f64, t6219: f64, t65884: f64, t66622: f64, t66668: f64, t73142: f64, t75836: f64, t78689: f64, t78713: f64, t78734: f64, t78775: f64, t79024: f64, t79056: f64, t79087: f64, t79120: f64, t79160: f64, t79188: f64, t79214: f64, t79251: f64, t6243: f64, t1751: f64, t22298: f64, t491: f64, t78757: f64, t6224: f64, t6238: f64, t11914: f64, t11915: f64, t1244: f64, t1246: f64, t15245: f64, t1734: f64, t1755: f64, t1756: f64, t19201: f64, t22243: f64, t22327: f64, t22354: f64, t22355: f64, t22389: f64, t3610: f64, t3612: f64, t3624: f64, t3625: f64, t6218: f64, t6252: f64, t6253: f64, t6257: f64, t65254: f64, t73630: f64, t11881: f64, t11883: f64, t11888: f64, t15027: f64, t1729: f64, t22349: f64, t22358: f64, t22368: f64, t22369: f64, t22375: f64, t22387: f64, t3508: f64, t44753: f64, t44754: f64, t44785: f64, t44786: f64, t470: f64, t493: f64, t5064: f64, t53592: f64, t53613: f64, t6256: f64, t6260: f64, t6739: f64, t11606: f64, t11889: f64, t1238: f64, t1241: f64, t1720: f64, t1758: f64, t1761: f64, t19232: f64, t19249: f64, t22008: f64, t22114: f64, t22341: f64, t22361: f64, t22365: f64, t22372: f64, t22386: f64, t22390: f64, t22394: f64, t44698: f64, t44701: f64, t44724: f64, t44726: f64, t45350: f64, t466: f64, t494: f64, t4945: f64, t498: f64, t5055: f64, t53565: f64, t6168: f64, t6244: f64, t6261: f64, t6263: f64, t6265: f64, t6267: f64, t6268: f64, t65262: f64, t73613: f64, t73856: f64, t73891: f64, t79008: f64, t1256: f64, t1763: f64, t193: f64, t336: f64, t43706: f64, t4700: f64, t71101: f64, t78344: f64, t78348: f64, t78355: f64, t78357: f64, t78359: f64, t78361: f64, t78364: f64, t78367: f64, t78370: f64, t78373: f64, t78646: f64, t79005: f64, t28: f64, t265: f64, t504: f64, t76559: f64, t78240: f64, t78305: f64, t78342: f64, t1409: f64, t1534: f64, t1649: f64, t1768: f64, t20217: f64, t20390: f64, t21076: f64, t22414: f64, t506: f64, t52: f64, t5398: f64, t5669: f64, t5966: f64, t6279: f64, t77953: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t5389: f64, t5445: f64, t1411: f64, t1426: f64, t1427: f64, t1434: f64, t19420: f64, t19430: f64, t20210: f64, t20218: f64, t20219: f64, t20285: f64, t2291: f64, t2298: f64, t31: f64, t39096: f64, t39114: f64, t4007: f64, t4012: f64, t5392: f64, t5393: f64, t5403: f64, t5427: f64, t5428: f64, t5442: f64, t634: f64, t638: f64, t65: f64, t66: f64, t72: f64, t80: f64, t1420: f64, t1423: f64, t19368: f64, t19390: f64, t20246: f64, t20255: f64, t20258: f64, t20261: f64, t2267: f64, t2274: f64, t39: f64, t39159: f64, t39168: f64, t39210: f64, t3981: f64, t3990: f64, t43: f64, t51: f64, t5416: f64, t5421: f64, t5424: f64, t55: f64, t56: f64, t78505: f64) -> (f64, f64, f64, f64, f64) {
        let (t79260, t79282) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1472(t225, t78637, t11546, t1174, t15569, t15740, t1653, t1726, t22162, t22244, t22280, t22288, t3440, t3577, t3578, t45112, t484, t488, t52628, t52879, t53274, t66500, t68, t73043, t73113, t78035, t78039);
        let t79320 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1473(t1174, t1177, t1196, t1227, t1735, t18321, t21758, t22129, t22133, t22137, t22197, t22258, t3560, t3577, t45128, t4582, t4889, t4987, t5024, t6184, t6188, t73076, t75847, t75912, t77621, t78043, t78047, t974);
        let t79349 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1474(t11678, t1214, t1735, t19083, t21776, t22012, t22185, t22309, t248, t3577, t3578, t44725, t44863, t45250, t4889, t5024, t53238, t53440, t5979, t6203, t6225, t66545, t73084, t73096, t73099, t73102, t79018);
        let t79387 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1475(t1174, t15569, t18321, t22119, t22154, t3555, t3577, t3578, t44805, t44817, t44938, t4889, t53490, t5975, t5979, t6178, t6192, t6219, t65884, t66622, t66668, t73142, t75836, t75847, t974);
        let t79391 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1476(t78689, t78713, t78734, t78775, t79024, t79056, t79087, t79120, t79160, t79188, t79214, t79251, t79282, t79320, t79349, t79387);
        let (t79398, t79410, t79453, t79461, t79467) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1477(t6243, t1751, t22298, t491, t78757, t6224, t6238, t11914, t11915, t1244, t1246, t15245, t1734, t1755, t1756, t19201, t22243, t22327, t22354, t22355, t22389, t3610, t3612, t3624, t3625, t6218, t6252, t6253, t6257, t65254, t73630);
        let (t79473, t79524) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1478(t491, t79018, t11881, t11883, t11888, t15027, t1729, t22349, t22358, t22368, t22369, t22375, t22387, t3508, t3610, t44753, t44754, t44785, t44786, t470, t493, t5064, t53592, t53613, t6224, t6256, t6260, t6739, t79391, t79410);
        let t79533 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1479(t11606, t11881, t11883, t11888, t11889, t1238, t1241, t1244, t1246, t15027, t15245, t1720, t1751, t1758, t1761, t19201, t19232, t19249, t22008, t22114, t22243, t22327, t22341, t22354, t22361, t22365, t22372, t22386, t22390, t22394, t3610, t3612, t3624, t44698, t44701, t44724, t44726, t45350, t466, t491, t494, t4945, t498, t5055, t5064, t53565, t6168, t6218, t6238, t6243, t6244, t6252, t6261, t6263, t6265, t6267, t6268, t65262, t73613, t73856, t73891, t79008, t79260, t79391, t79398, t79410, t79453, t79461, t79467, t79473, t79524);
        let t79538 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1480(t1256, t1763, t193, t336, t43706, t4700, t71101, t78344, t78348, t78355, t78357, t78359, t78361, t78364, t78367, t78370, t78373, t78646, t79005, t79533);
        let t79553 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1481(t28, t265, t504, t76559, t78240, t78305, t78342, t79538, t1409, t1534, t1649, t1768, t20217, t20390, t21076, t22414, t506, t52, t5398, t5669, t5966, t6279, t75912, t77953, dens_threshold, rho1, zeta_threshold);
        let (t79579, t79585, t79637) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1482(t5389, t5445, t1411, t1426, t1427, t1434, t19420, t19430, t20210, t20217, t20218, t20219, t20285, t2291, t2298, t31, t39096, t39114, t4007, t4012, t5392, t5393, t5398, t5403, t5427, t5428, t5442, t634, t638, t65, t66, t72, t75836, t75847, t75912, t80);
        let t79692 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1483(t1420, t1423, t19368, t19390, t20217, t20246, t20255, t20258, t20261, t2267, t2274, t39, t39159, t39168, t39210, t3981, t3990, t43, t51, t5398, t5416, t5421, t5424, t55, t56, t75836, t75847, t75912, t78505);
    (t79553, t79579, t79585, t79637, t79692)
}
