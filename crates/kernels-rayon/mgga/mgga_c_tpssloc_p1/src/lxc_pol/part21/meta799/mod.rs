//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta799 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2779;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2780;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2781;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2782;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2783;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2784;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2785;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta799(t16673: f64, t2696: f64, t849: f64, t13360: f64, t4261: f64, t5584: f64, t9975: f64, t5619: f64, t9674: f64, t4250: f64, t46657: f64, t16907: f64, t9638: f64, t119: f64, t12971: f64, t13222: f64, t13229: f64, t13254: f64, t13262: f64, t13347: f64, t13365: f64, t1484: f64, t1516: f64, t16901: f64, t16932: f64, t16937: f64, t16946: f64, t210: f64, t2553: f64, t2623: f64, t2643: f64, t2645: f64, t2684: f64, t2701: f64, t4172: f64, t4191: f64, t46570: f64, t47037: f64, t47044: f64, t5527: f64, t58139: f64, t787: f64, t820: f64, t843: f64, t9607: f64, t17013: f64, t13258: f64, t16845: f64, t13261: f64, t4166: f64, t13151: f64, t13156: f64, t13164: f64, t13191: f64, t16723: f64, t16729: f64, t16737: f64, t16749: f64, t1891: f64, t228: f64, t2379: f64, t2667: f64, t2671: f64, t2675: f64, t4219: f64, t4225: f64, t4227: f64, t4230: f64, t5544: f64, t5601: f64, t5605: f64, t5608: f64, t58090: f64, t68: f64, t822: f64, t824: f64, t825: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40626: f64, t57877: f64, t57879: f64, t57880: f64, t57885: f64, t57886: f64, t57888: f64, t57889: f64, t57891: f64, t57892: f64, t57897: f64, t57898: f64, t57899: f64, t39373: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40679: f64, t40685: f64, t40708: f64, t40714: f64, t40716: f64, t57900: f64, t57903: f64, t57907: f64, t57908: f64, t57936: f64, t57939: f64, t57943: f64, t57946: f64, t57948: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t40721: f64, t40732: f64, t57959: f64, t57961: f64, t57962: f64, t57966: f64, t57970: f64, t57972: f64, t57975: f64, t57983: f64, t57986: f64, t57987: f64, t57988: f64, t57989: f64, t57990: f64, t39483: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t57993: f64, t57996: f64, t58005: f64, t58008: f64, t58020: f64, t58022: f64, t58023: f64, t58025: f64, t58026: f64, t58027: f64, t58028: f64, t58030: f64, t58032: f64, t58033: f64, t58034: f64, t39529: f64, t40764: f64, t40766: f64, t40779: f64, t40784: f64, t40790: f64, t40793: f64, t40797: f64, t40799: f64, t58035: f64, t58040: f64, t58042: f64, t58046: f64, t58048: f64, t58053: f64, t58055: f64, t58056: f64, t58058: f64, t58059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58845, t58847, t58853, t58859, t58873, t58885) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2779(t16673, t2696, t849, t13360, t4261, t5584, t9975, t5619, t9674, t4250, t46657, t16907, t9638);
        let t58887 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2780(t119, t12971, t13222, t13229, t13254, t13262, t13347, t13365, t1484, t1516, t16901, t16932, t16937, t16946, t210, t2553, t2623, t2643, t2645, t2684, t2701, t4172, t4191, t4261, t46570, t47037, t47044, t5527, t58139, t58845, t58847, t58853, t58859, t58873, t58885, t787, t820, t843, t9607);
        let (t58890, t58900, t58904, t58947) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2781(t17013, t9638, t13258, t16845, t13261, t4166, t13151, t13156, t13164, t13191, t16723, t16729, t16737, t16749, t1891, t228, t2379, t2667, t2671, t2675, t4219, t4225, t4227, t4230, t5544, t5601, t5605, t5608, t58090, t58139, t68, t822, t824, t825);
        let t58963 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2782(t39249, t39256, t39309, t39312, t39316, t39320, t40626, t57877, t57879, t57880, t57885, t57886, t57888, t57889, t57891, t57892, t57897, t57898, t57899);
        let t58964 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2783(t39373, t39397, t39400, t39408, t39411, t40679, t40685, t40708, t40714, t40716, t57900, t57903, t57907, t57908, t57936, t57939, t57943, t57946, t57948);
        let t58966 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2784(t39463, t39468, t39472, t39476, t40721, t40732, t57959, t57961, t57962, t57966, t57970, t57972, t57975, t57983, t57986, t57987, t57988, t57989, t57990);
        let t58967 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2785(t39483, t40741, t40743, t40748, t40760, t57993, t57996, t58005, t58008, t58020, t58022, t58023, t58025, t58026, t58027, t58028, t58030, t58032, t58033, t58034);
        let t58970 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2786(t39529, t40764, t40766, t40779, t40784, t40790, t40793, t40797, t40799, t58035, t58040, t58042, t58046, t58048, t58053, t58055, t58056, t58058, t58059);
    (t58887, t58890, t58900, t58904, t58947, t58963, t58964, t58966, t58967, t58970)
}
