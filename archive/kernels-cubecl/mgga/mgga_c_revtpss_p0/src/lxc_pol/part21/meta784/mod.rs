//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta784 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2821;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2822;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2823;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2824;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2825;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta784<F: Float>(t2394: F, t890: F, t198: F, t14353: F, t40167: F, t40171: F, t40184: F, t4541: F, t4553: F, t50874: F, t50875: F, t50876: F, t50879: F, t50881: F, t50884: F, t2832: F, t11064: F, t14436: F, t1940: F, t2403: F, t2408: F, t2430: F, t41161: F, t4537: F, t4556: F, t50887: F, t50889: F, t50891: F, t50892: F, t50894: F, t50897: F, t50898: F, t14397: F, t40076: F, t40079: F, t40194: F, t40198: F, t50899: F, t50900: F, t50902: F, t50905: F, t50907: F, t10627: F, t10818: F, t11075: F, t14375: F, t14749: F, t14767: F, t1544: F, t1583: F, t2404: F, t39419: F, t39422: F, t39429: F, t39432: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t39773: F, t40067: F, t40072: F, t40099: F, t40103: F, t40115: F, t41137: F, t4343: F, t4546: F, t49865: F, t49867: F, t49868: F, t49869: F, t49870: F, t49903: F, t49912: F, t49913: F, t49921: F, t49925: F, t49927: F, t49930: F, t49941: F, t49944: F, t49945: F, t49972: F, t49988: F, t50040: F, t50045: F, t50046: F, t50048: F, t50051: F, t50055: F, t50056: F, t50078: F, t50102: F, t50861: F, t50864: F, t50866: F, t50869: F, t50871: F, t50872: F, t51769: F, t892: F, t2: F, t2838: F, t580: F, t895: F, t15091: F, t22: F, t265: F, t4567: F, t588: F, t15234: F, t2986: F, t974: F, t981: F, t11506: F, t15542: F, t4707: F, t15538: F, t3022: F, t10356: F, t15153: F, t128: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51775, t51786) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2821::<F>(t2394, t890, t198, t14353, t40167, t40171, t40184, t4541, t4553, t50874, t50875, t50876, t50879, t50881, t50884);
        let t51802 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2822::<F>(t2832, t890, t11064, t14353, t14436, t1940, t2403, t2408, t2430, t41161, t4537, t4556, t50887, t50889, t50891, t50892, t50894, t50897, t50898);
        let t51810 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2823::<F>(t2430, t890, t14397, t1940, t2403, t2832, t40076, t40079, t40194, t40198, t4556, t50899, t50900, t50902, t50905, t50907);
        let t51814 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2824::<F>(t10627, t10818, t11075, t14375, t14749, t14767, t1544, t1583, t198, t2403, t2404, t39419, t39422, t39429, t39432, t39483, t39520, t39528, t39531, t39534, t39537, t39540, t39773, t40067, t40072, t40099, t40103, t40115, t41137, t4343, t4541, t4546, t4556, t49865, t49867, t49868, t49869, t49870, t49903, t49912, t49913, t49921, t49925, t49927, t49930, t49941, t49944, t49945, t49972, t49988, t50040, t50045, t50046, t50048, t50051, t50055, t50056, t50078, t50102, t50861, t50864, t50866, t50869, t50871, t50872, t51769, t51775, t51786, t51802, t51810, t892);
        let (t51827, t51829, t51831, t51833, t51835, t51840) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2825::<F>(t2, t2838, t580, t895, t15091, t22, t265, t4567, t588, t15234, t2986, t974, t981);
        let (t51844, t51846, t51847, t51849) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2826::<F>(t11506, t15542, t4707, t981, t15538, t3022, t10356, t15153, t128, t904);
    (t51814, t51827, t51829, t51831, t51833, t51835, t51840, t51844, t51846, t51847, t51849)
}
