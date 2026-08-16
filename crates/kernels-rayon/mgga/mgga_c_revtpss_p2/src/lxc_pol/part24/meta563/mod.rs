//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1695;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1696;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1697;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1698;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1699;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1700;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta563(t1668: f64, t24031: f64, t88004: f64, t88007: f64, t88012: f64, t88016: f64, t88023: f64, t88026: f64, t88028: f64, t88030: f64, t88034: f64, t88036: f64, t88038: f64, t88041: f64, t88046: f64, t88048: f64, t88050: f64, t88052: f64, t88054: f64, t88140: f64, t88358: f64, t88361: f64, t88363: f64, t88368: f64, t88573: f64, t88432: f64, t88445: f64, t88448: f64, t88451: f64, t88481: f64, t88580: f64, t88584: f64, t88586: f64, t88588: f64, t88590: f64, t88592: f64, t88596: f64, t23714: f64, t4724: f64, t981: f64, t4711: f64, t78429: f64, t23446: f64, t4719: f64, t23453: f64, t19049: f64, t6219: f64, t88510: f64, t88562: f64, t88564: f64, t88567: f64, t88600: f64, t88602: f64, t88607: f64, t1041: f64, t1042: f64, t1045: f64, t1592: f64, t16067: f64, t16089: f64, t16199: f64, t19450: f64, t19968: f64, t23830: f64, t23839: f64, t23929: f64, t23934: f64, t23964: f64, t3092: f64, t3117: f64, t3127: f64, t357: f64, t373: f64, t42121: f64, t43291: f64, t4801: f64, t4892: f64, t4899: f64, t54500: f64, t54564: f64, t6299: f64, t6308: f64, t6331: f64, t65339: f64, t78873: f64, t79071: f64, t88901: f64, t88925: f64, t6244: f64, t1011: f64, t11774: f64, t11927: f64, t15696: f64, t15926: f64, t19611: f64, t23903: f64, t23912: f64, t23916: f64, t23999: f64, t3091: f64, t42328: f64, t43069: f64, t4872: f64, t4919: f64, t55122: f64, t5825: f64, t6258: f64, t6267: f64, t66306: f64, t79107: f64, t79112: f64, t79139: f64, t79141: f64, t79155: f64, t88132: f64, t88828: f64, t6305: f64, t1063: f64, t1066: f64, t11632: f64, t1469: f64, t15689: f64, t16012: f64, t16226: f64, t23907: f64, t247: f64, t3155: f64, t42472: f64, t42621: f64, t43050: f64, t4893: f64, t4915: f64, t6266: f64, t66777: f64, t67052: f64, t79219: f64, t79233: f64, t79253: f64, t79450: f64, t88087: f64, t88095: f64, t88102: f64, t88106: f64, t88116: f64, t88794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t88948, t88980) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1695(t1668, t24031, t88004, t88007, t88012, t88016, t88023, t88026, t88028, t88030, t88034, t88036, t88038);
        let t88981 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1696(t88041, t88046, t88048, t88050, t88052, t88054, t88140, t88358, t88361, t88363, t88368, t88573);
        let t88983 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1697(t88432, t88445, t88448, t88451, t88481, t88580, t88584, t88586, t88588, t88590, t88592, t88596);
        let (t88986, t88989, t88991, t88993, t88995, t88996) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1698(t23714, t4724, t981, t4711, t78429, t23446, t4719, t23453, t19049, t6219, t88510, t88562, t88564, t88567, t88600, t88602, t88607);
        let (t88998, t89009) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1699(t88980, t88981, t88983, t88996, t1041, t1042, t1045, t1592, t16067, t16089, t16199, t19450, t19968, t23830, t23839, t23929, t23934, t23964, t3092, t3117, t3127, t357, t373, t42121, t43291, t4801, t4892, t4899, t54500, t54564, t6299, t6308, t6331, t65339, t78873, t79071, t88901, t88925, t88948);
        let (t89035, t89046) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1700(t6244, t6299, t1011, t1042, t1045, t11774, t11927, t15696, t15926, t19611, t23903, t23912, t23916, t23999, t3091, t3092, t3117, t3127, t42328, t43069, t4872, t4919, t55122, t5825, t6258, t6267, t66306, t79107, t79112, t79139, t79141, t79155, t88132, t88828);
        let (t89084, t89094) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1701(t6244, t6305, t1011, t1063, t1066, t11632, t11774, t1469, t15689, t15696, t16012, t16226, t23907, t247, t3117, t3155, t42472, t42621, t43050, t4893, t4915, t6266, t6267, t66777, t67052, t79219, t79233, t79253, t79450, t88087, t88095, t88102, t88106, t88116, t88794);
    (t88948, t88986, t88989, t88991, t88993, t88995, t88998, t89009, t89035, t89046, t89084, t89094)
}
