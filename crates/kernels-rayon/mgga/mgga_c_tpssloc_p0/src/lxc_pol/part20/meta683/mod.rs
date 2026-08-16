//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2584;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2585;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2586;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2587;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2588;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2589;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2590;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta683(t1174: f64, t457: f64, t4936: f64, t698: f64, t15277: f64, t3431: f64, t15281: f64, t15303: f64, t11540: f64, t4889: f64, t11529: f64, t4912: f64, t11549: f64, t44586: f64, t44589: f64, t44592: f64, t44595: f64, t44602: f64, t44628: f64, t44631: f64, t44635: f64, t44638: f64, t44641: f64, t460: f64, t52327: f64, t52345: f64, t974: f64, t51993: f64, t52047: f64, t52094: f64, t52150: f64, t52197: f64, t52257: f64, t52303: f64, t15814: f64, t225: f64, t11720: f64, t1751: f64, t3030: f64, t4940: f64, t3623: f64, t1009: f64, t15425: f64, t1243: f64, t50816: f64, t50818: f64, t50821: f64, t51111: f64, t51113: f64, t51119: f64, t51122: f64, t51124: f64, t51126: f64, t51128: f64, t51131: f64, t51133: f64, t51245: f64, t51248: f64, t51251: f64, t51793: f64, t51795: f64, t51797: f64, t51800: f64, t51802: f64, t51399: f64, t51401: f64, t51404: f64, t51437: f64, t51439: f64, t51441: f64, t51443: f64, t51446: f64, t51449: f64, t51453: f64, t51456: f64, t51459: f64, t51463: f64, t51466: f64, t51806: f64, t51809: f64, t51814: f64, t51818: f64, t51822: f64, t51824: f64, t51470: f64, t51472: f64, t51474: f64, t51476: f64, t51478: f64, t51480: f64, t51482: f64, t51485: f64, t51549: f64, t51593: f64, t51831: f64, t51833: f64, t51835: f64, t51839: f64, t51844: f64, t51847: f64, t51851: f64, t51853: f64, t51855: f64, t51857: f64, t51738: f64, t51741: f64, t51744: f64, t51884: f64, t51889: f64, t51892: f64, t51898: f64, t51903: f64, t51905: f64, t51913: f64, t51916: f64, t51641: f64, t51669: f64, t51736: f64, t51859: f64, t51862: f64, t51864: f64, t51866: f64, t51870: f64, t51874: f64, t51880: f64, t11638: f64, t11888: f64, t11910: f64, t11914: f64, t11915: f64, t1244: f64, t1246: f64, t1247: f64, t14988: f64, t15245: f64, t15247: f64, t1755: f64, t23508: f64, t3610: f64, t3624: f64, t3626: f64, t44785: f64, t475: f64, t491: f64, t494: f64, t5068: f64, t5072: f64, t5079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52355, t52357, t52362, t52364, t52367) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2584(t1174, t457, t4936, t698, t15277, t3431, t15281, t15303, t11540, t4889, t11529, t4912);
        let t52374 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2585(t52367, t11549, t1174, t44586, t44589, t44592, t44595, t44602, t44628, t44631, t44635, t44638, t44641, t457, t460, t4889, t52327, t52345, t52355, t52357, t52362, t52364, t974);
        let (t52377, t52386, t52424) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2586(t51993, t52047, t52094, t52150, t52197, t52257, t52303, t52374, t15814, t225, t11720, t1751);
        let (t52434, t52435, t52446, t52447, t52450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2587(t3030, t4940, t3623, t1009, t15425, t1243, t50816, t50818, t50821, t51111, t51113, t51119, t51122, t51124, t51126, t51128, t51131, t51133, t51245, t51248, t51251, t51793, t51795, t51797, t51800, t51802);
        let t52451 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2588(t51399, t51401, t51404, t51437, t51439, t51441, t51443, t51446, t51449, t51453, t51456, t51459, t51463, t51466, t51806, t51809, t51814, t51818, t51822, t51824);
        let t52453 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2589(t51470, t51472, t51474, t51476, t51478, t51480, t51482, t51485, t51549, t51593, t51831, t51833, t51835, t51839, t51844, t51847, t51851, t51853, t51855, t51857);
        let t52458 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2590(t51738, t51741, t51744, t51884, t51889, t51892, t51898, t51903, t51905, t51913, t51916, t51641, t51669, t51736, t51859, t51862, t51864, t51866, t51870, t51874, t51880, t52450, t52451, t52453);
        let (t52462, t52471) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2591(t225, t52377, t11638, t11720, t11888, t11910, t11914, t11915, t1244, t1246, t1247, t14988, t15245, t15247, t1751, t1755, t23508, t3610, t3624, t3626, t44785, t475, t491, t494, t5068, t5072, t5079, t52424, t52435, t52447, t52458);
    (t52377, t52386, t52424, t52434, t52446, t52458, t52462, t52471)
}
