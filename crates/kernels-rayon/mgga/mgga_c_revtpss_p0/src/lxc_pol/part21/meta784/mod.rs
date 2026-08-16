//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta784 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2821;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2822;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2823;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2824;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2825;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta784(t2394: f64, t890: f64, t198: f64, t14353: f64, t40167: f64, t40171: f64, t40184: f64, t4541: f64, t4553: f64, t50874: f64, t50875: f64, t50876: f64, t50879: f64, t50881: f64, t50884: f64, t2832: f64, t11064: f64, t14436: f64, t1940: f64, t2403: f64, t2408: f64, t2430: f64, t41161: f64, t4537: f64, t4556: f64, t50887: f64, t50889: f64, t50891: f64, t50892: f64, t50894: f64, t50897: f64, t50898: f64, t14397: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t50899: f64, t50900: f64, t50902: f64, t50905: f64, t50907: f64, t10627: f64, t10818: f64, t11075: f64, t14375: f64, t14749: f64, t14767: f64, t1544: f64, t1583: f64, t2404: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39773: f64, t40067: f64, t40072: f64, t40099: f64, t40103: f64, t40115: f64, t41137: f64, t4343: f64, t4546: f64, t49865: f64, t49867: f64, t49868: f64, t49869: f64, t49870: f64, t49903: f64, t49912: f64, t49913: f64, t49921: f64, t49925: f64, t49927: f64, t49930: f64, t49941: f64, t49944: f64, t49945: f64, t49972: f64, t49988: f64, t50040: f64, t50045: f64, t50046: f64, t50048: f64, t50051: f64, t50055: f64, t50056: f64, t50078: f64, t50102: f64, t50861: f64, t50864: f64, t50866: f64, t50869: f64, t50871: f64, t50872: f64, t51769: f64, t892: f64, t2: f64, t2838: f64, t580: f64, t895: f64, t15091: f64, t22: f64, t265: f64, t4567: f64, t588: f64, t15234: f64, t2986: f64, t974: f64, t981: f64, t11506: f64, t15542: f64, t4707: f64, t15538: f64, t3022: f64, t10356: f64, t15153: f64, t128: f64, t904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51775, t51786) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2821(t2394, t890, t198, t14353, t40167, t40171, t40184, t4541, t4553, t50874, t50875, t50876, t50879, t50881, t50884);
        let t51802 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2822(t2832, t890, t11064, t14353, t14436, t1940, t2403, t2408, t2430, t41161, t4537, t4556, t50887, t50889, t50891, t50892, t50894, t50897, t50898);
        let t51810 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2823(t2430, t890, t14397, t1940, t2403, t2832, t40076, t40079, t40194, t40198, t4556, t50899, t50900, t50902, t50905, t50907);
        let t51814 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2824(t10627, t10818, t11075, t14375, t14749, t14767, t1544, t1583, t198, t2403, t2404, t39419, t39422, t39429, t39432, t39483, t39520, t39528, t39531, t39534, t39537, t39540, t39773, t40067, t40072, t40099, t40103, t40115, t41137, t4343, t4541, t4546, t4556, t49865, t49867, t49868, t49869, t49870, t49903, t49912, t49913, t49921, t49925, t49927, t49930, t49941, t49944, t49945, t49972, t49988, t50040, t50045, t50046, t50048, t50051, t50055, t50056, t50078, t50102, t50861, t50864, t50866, t50869, t50871, t50872, t51769, t51775, t51786, t51802, t51810, t892);
        let (t51827, t51829, t51831, t51833, t51835, t51840) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2825(t2, t2838, t580, t895, t15091, t22, t265, t4567, t588, t15234, t2986, t974, t981);
        let (t51844, t51846, t51847, t51849) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2826(t11506, t15542, t4707, t981, t15538, t3022, t10356, t15153, t128, t904);
    (t51814, t51827, t51829, t51831, t51833, t51835, t51840, t51844, t51846, t51847, t51849)
}
