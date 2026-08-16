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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2584;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2585;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2586;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2587;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2588;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2589;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2590;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta683<F: Float>(t1174: F, t457: F, t4936: F, t698: F, t15277: F, t3431: F, t15281: F, t15303: F, t11540: F, t4889: F, t11529: F, t4912: F, t11549: F, t44586: F, t44589: F, t44592: F, t44595: F, t44602: F, t44628: F, t44631: F, t44635: F, t44638: F, t44641: F, t460: F, t52327: F, t52345: F, t974: F, t51993: F, t52047: F, t52094: F, t52150: F, t52197: F, t52257: F, t52303: F, t15814: F, t225: F, t11720: F, t1751: F, t3030: F, t4940: F, t3623: F, t1009: F, t15425: F, t1243: F, t50816: F, t50818: F, t50821: F, t51111: F, t51113: F, t51119: F, t51122: F, t51124: F, t51126: F, t51128: F, t51131: F, t51133: F, t51245: F, t51248: F, t51251: F, t51793: F, t51795: F, t51797: F, t51800: F, t51802: F, t51399: F, t51401: F, t51404: F, t51437: F, t51439: F, t51441: F, t51443: F, t51446: F, t51449: F, t51453: F, t51456: F, t51459: F, t51463: F, t51466: F, t51806: F, t51809: F, t51814: F, t51818: F, t51822: F, t51824: F, t51470: F, t51472: F, t51474: F, t51476: F, t51478: F, t51480: F, t51482: F, t51485: F, t51549: F, t51593: F, t51831: F, t51833: F, t51835: F, t51839: F, t51844: F, t51847: F, t51851: F, t51853: F, t51855: F, t51857: F, t51738: F, t51741: F, t51744: F, t51884: F, t51889: F, t51892: F, t51898: F, t51903: F, t51905: F, t51913: F, t51916: F, t51641: F, t51669: F, t51736: F, t51859: F, t51862: F, t51864: F, t51866: F, t51870: F, t51874: F, t51880: F, t11638: F, t11888: F, t11910: F, t11914: F, t11915: F, t1244: F, t1246: F, t1247: F, t14988: F, t15245: F, t15247: F, t1755: F, t23508: F, t3610: F, t3624: F, t3626: F, t44785: F, t475: F, t491: F, t494: F, t5068: F, t5072: F, t5079: F) -> (F, F, F, F, F, F, F, F) {
        let (t52355, t52357, t52362, t52364, t52367) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2584::<F>(t1174, t457, t4936, t698, t15277, t3431, t15281, t15303, t11540, t4889, t11529, t4912);
        let t52374 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2585::<F>(t52367, t11549, t1174, t44586, t44589, t44592, t44595, t44602, t44628, t44631, t44635, t44638, t44641, t457, t460, t4889, t52327, t52345, t52355, t52357, t52362, t52364, t974);
        let (t52377, t52386, t52424) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2586::<F>(t51993, t52047, t52094, t52150, t52197, t52257, t52303, t52374, t15814, t225, t11720, t1751);
        let (t52434, t52435, t52446, t52447, t52450) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2587::<F>(t3030, t4940, t3623, t1009, t15425, t1243, t50816, t50818, t50821, t51111, t51113, t51119, t51122, t51124, t51126, t51128, t51131, t51133, t51245, t51248, t51251, t51793, t51795, t51797, t51800, t51802);
        let t52451 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2588::<F>(t51399, t51401, t51404, t51437, t51439, t51441, t51443, t51446, t51449, t51453, t51456, t51459, t51463, t51466, t51806, t51809, t51814, t51818, t51822, t51824);
        let t52453 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2589::<F>(t51470, t51472, t51474, t51476, t51478, t51480, t51482, t51485, t51549, t51593, t51831, t51833, t51835, t51839, t51844, t51847, t51851, t51853, t51855, t51857);
        let t52458 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2590::<F>(t51738, t51741, t51744, t51884, t51889, t51892, t51898, t51903, t51905, t51913, t51916, t51641, t51669, t51736, t51859, t51862, t51864, t51866, t51870, t51874, t51880, t52450, t52451, t52453);
        let (t52462, t52471) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2591::<F>(t225, t52377, t11638, t11720, t11888, t11910, t11914, t11915, t1244, t1246, t1247, t14988, t15245, t15247, t1751, t1755, t23508, t3610, t3624, t3626, t44785, t475, t491, t494, t5068, t5072, t5079, t52424, t52435, t52447, t52458);
    (t52377, t52386, t52424, t52434, t52446, t52458, t52462, t52471)
}
