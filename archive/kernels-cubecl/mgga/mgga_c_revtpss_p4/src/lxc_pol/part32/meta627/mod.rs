//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2002;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2003;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2004;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2005;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2006;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2007;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta627<F: Float>(t108978: F, t2047: F, t108986: F, t101230: F, t101785: F, t101955: F, t101969: F, t101972: F, t10309: F, t108966: F, t108975: F, t108983: F, t108990: F, t25162: F, t26175: F, t26182: F, t28147: F, t28628: F, t34764: F, t5: F, t109895: F, t109918: F, t109945: F, t109970: F, t109992: F, t110012: F, t110027: F, t117: F, t108126: F, t109263: F, t109368: F, t109399: F, t109423: F, t109446: F, t109467: F, t109493: F, t109516: F, t109533: F, t109563: F, t109598: F, t109628: F, t109656: F, t109681: F, t109704: F, t109724: F, t109756: F, t109864: F, t109874: F, t1310: F, t13426: F, t1450: F, t18227: F, t2014: F, t2089: F, t21881: F, t21882: F, t25082: F, t26405: F, t28196: F, t28286: F, t28727: F, t28750: F, t28935: F, t29498: F, t30209: F, t30511: F, t30553: F, t34251: F, t4248: F, t4254: F, t4293: F, t508: F, t532: F, t5517: F, t649: F, t651: F, t7359: F, t7898: F, t7983: F, t7988: F, t95472: F, t108688: F, t1518: F, t18235: F, t18245: F, t2056: F, t2322: F, t27123: F, t27126: F, t28586: F, t28696: F, t28760: F, t29508: F, t30570: F, t30571: F, t30578: F, t4292: F, t6765: F, t7367: F, t7373: F, t7374: F, t7378: F, t75439: F, t7732: F, t7978: F, t8065: F, t85360: F, t116: F, t30552: F, t1940: F, t2255: F, t8020: F, t105928: F, t28472: F, t105902: F, t105909: F, t106510: F, t18280: F, t2071: F, t2403: F, t27169: F, t27402: F, t28456: F, t28460: F, t29591: F, t29602: F, t29606: F, t29713: F, t30420: F, t4541: F, t7010: F, t7428: F, t7432: F, t7749: F, t95976: F, t198: F, t8034: F, t2411: F, t30419: F, t105898: F, t105919: F, t105924: F, t106555: F, t106566: F, t106569: F, t106611: F, t106618: F, t106626: F, t26425: F, t26585: F, t27173: F, t27385: F, t28291: F, t29716: F, t30317: F, t50080: F, t5824: F, t7092: F, t105936: F, t95822: F, t102930: F, t102934: F, t102937: F, t102939: F, t102941: F, t102943: F, t102945: F, t1579: F, t18784: F, t2061: F, t25383: F, t28340: F, t29698: F, t30342: F, t4533: F, t6071: F, t7070: F, t7071: F, t7398: F, t7424: F, t7997: F, t212: F, t30379: F, t689: F, t780: F, t95537: F, t213: F, t102947: F, t102953: F, t102956: F, t102964: F, t102969: F, t103424: F, t25317: F, t25391: F, t27312: F, t6048: F, t887: F, t95542: F, t95548: F, t95551: F, t95562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t110049 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2002::<F>(t108978, t2047, t108986, t101230, t101785, t101955, t101969, t101972, t10309, t108966, t108975, t108983, t108990, t25162, t26175, t26182, t28147, t28628, t34764);
        let (t110054, t110058) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2003::<F>(t5, t109895, t109918, t109945, t109970, t109992, t110012, t110027, t110049, t117, t108126, t109263, t109368, t109399, t109423, t109446, t109467, t109493, t109516, t109533, t109563, t109598, t109628, t109656, t109681, t109704, t109724, t109756, t109864, t109874, t1310, t13426, t1450, t18227, t2014, t2089, t21881, t21882, t25082, t26405, t28196, t28286, t28727, t28750, t28935, t29498, t30209, t30511, t30553, t34251, t4248, t4254, t4293, t508, t532, t5517, t649, t651, t7359, t7898, t7983, t7988, t95472);
        let t110102 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2004::<F>(t108688, t1310, t1518, t18235, t18245, t2056, t2322, t27123, t27126, t28196, t28286, t28586, t28696, t28760, t29508, t30570, t30571, t30578, t4248, t4254, t4292, t651, t6765, t7359, t7367, t7373, t7374, t7378, t75439, t7732, t7978, t8065, t85360);
        let (t110110, t110150, t110154, t110158) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2005::<F>(t116, t30552, t1940, t2255, t8020, t105928, t28472, t105902, t105909, t106510, t18280, t2071, t2403, t27169, t27402, t28456, t28460, t29591, t29602, t29606, t29713, t30420, t4541, t7010, t7428, t7432, t7749, t95976);
        let (t110165, t110177, t110196) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2006::<F>(t198, t8034, t2411, t30419, t105898, t105919, t105924, t106555, t106566, t106569, t106611, t106618, t106626, t1940, t2071, t2403, t26425, t26585, t27173, t27385, t28291, t28472, t29716, t30317, t50080, t5824, t7092, t7428, t8020);
        let t110242 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2007::<F>(t105936, t95822, t102930, t102934, t102937, t102939, t102941, t102943, t102945, t1579, t18784, t2061, t25383, t28340, t29698, t30342, t4533, t6071, t7070, t7071, t7398, t7424, t7997);
        let t110261 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2008::<F>(t212, t30379, t689, t780, t105936, t95537, t213, t102947, t102953, t102956, t102964, t102969, t103424, t25317, t25391, t27312, t6048, t7070, t7398, t887, t95542, t95548, t95551, t95562);
    (t110054, t110058, t110102, t110110, t110150, t110154, t110158, t110165, t110177, t110196, t110242, t110261)
}
