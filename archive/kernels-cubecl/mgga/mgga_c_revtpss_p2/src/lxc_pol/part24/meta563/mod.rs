//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta563 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1695;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1696;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1697;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1698;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1699;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1700;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta563<F: Float>(t1668: F, t24031: F, t88004: F, t88007: F, t88012: F, t88016: F, t88023: F, t88026: F, t88028: F, t88030: F, t88034: F, t88036: F, t88038: F, t88041: F, t88046: F, t88048: F, t88050: F, t88052: F, t88054: F, t88140: F, t88358: F, t88361: F, t88363: F, t88368: F, t88573: F, t88432: F, t88445: F, t88448: F, t88451: F, t88481: F, t88580: F, t88584: F, t88586: F, t88588: F, t88590: F, t88592: F, t88596: F, t23714: F, t4724: F, t981: F, t4711: F, t78429: F, t23446: F, t4719: F, t23453: F, t19049: F, t6219: F, t88510: F, t88562: F, t88564: F, t88567: F, t88600: F, t88602: F, t88607: F, t1041: F, t1042: F, t1045: F, t1592: F, t16067: F, t16089: F, t16199: F, t19450: F, t19968: F, t23830: F, t23839: F, t23929: F, t23934: F, t23964: F, t3092: F, t3117: F, t3127: F, t357: F, t373: F, t42121: F, t43291: F, t4801: F, t4892: F, t4899: F, t54500: F, t54564: F, t6299: F, t6308: F, t6331: F, t65339: F, t78873: F, t79071: F, t88901: F, t88925: F, t6244: F, t1011: F, t11774: F, t11927: F, t15696: F, t15926: F, t19611: F, t23903: F, t23912: F, t23916: F, t23999: F, t3091: F, t42328: F, t43069: F, t4872: F, t4919: F, t55122: F, t5825: F, t6258: F, t6267: F, t66306: F, t79107: F, t79112: F, t79139: F, t79141: F, t79155: F, t88132: F, t88828: F, t6305: F, t1063: F, t1066: F, t11632: F, t1469: F, t15689: F, t16012: F, t16226: F, t23907: F, t247: F, t3155: F, t42472: F, t42621: F, t43050: F, t4893: F, t4915: F, t6266: F, t66777: F, t67052: F, t79219: F, t79233: F, t79253: F, t79450: F, t88087: F, t88095: F, t88102: F, t88106: F, t88116: F, t88794: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t88948, t88980) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1695::<F>(t1668, t24031, t88004, t88007, t88012, t88016, t88023, t88026, t88028, t88030, t88034, t88036, t88038);
        let t88981 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1696::<F>(t88041, t88046, t88048, t88050, t88052, t88054, t88140, t88358, t88361, t88363, t88368, t88573);
        let t88983 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1697::<F>(t88432, t88445, t88448, t88451, t88481, t88580, t88584, t88586, t88588, t88590, t88592, t88596);
        let (t88986, t88989, t88991, t88993, t88995, t88996) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1698::<F>(t23714, t4724, t981, t4711, t78429, t23446, t4719, t23453, t19049, t6219, t88510, t88562, t88564, t88567, t88600, t88602, t88607);
        let (t88998, t89009) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1699::<F>(t88980, t88981, t88983, t88996, t1041, t1042, t1045, t1592, t16067, t16089, t16199, t19450, t19968, t23830, t23839, t23929, t23934, t23964, t3092, t3117, t3127, t357, t373, t42121, t43291, t4801, t4892, t4899, t54500, t54564, t6299, t6308, t6331, t65339, t78873, t79071, t88901, t88925, t88948);
        let (t89035, t89046) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1700::<F>(t6244, t6299, t1011, t1042, t1045, t11774, t11927, t15696, t15926, t19611, t23903, t23912, t23916, t23999, t3091, t3092, t3117, t3127, t42328, t43069, t4872, t4919, t55122, t5825, t6258, t6267, t66306, t79107, t79112, t79139, t79141, t79155, t88132, t88828);
        let (t89084, t89094) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1701::<F>(t6244, t6305, t1011, t1063, t1066, t11632, t11774, t1469, t15689, t15696, t16012, t16226, t23907, t247, t3117, t3155, t42472, t42621, t43050, t4893, t4915, t6266, t6267, t66777, t67052, t79219, t79233, t79253, t79450, t88087, t88095, t88102, t88106, t88116, t88794);
    (t88948, t88986, t88989, t88991, t88993, t88995, t88998, t89009, t89035, t89046, t89084, t89094)
}
