//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta679 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2127;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2128;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2129;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2130;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2131;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2132;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta679<F: Float>(t19451: F, t6535: F, t22574: F, t28830: F, t31035: F, t1390: F, t19631: F, t1983: F, t6878: F, t25989: F, t91655: F, t1845: F, t5356: F, t26161: F, t26162: F, t26114: F, t7468: F, t26179: F, t1266: F, t1980: F, t20098: F, t27996: F, t28811: F, t510: F, t5450: F, t650: F, t652: F, t671: F, t6862: F, t96655: F, t96796: F, t96799: F, t96802: F, t96805: F, t96807: F, t96813: F, t26003: F, t7458: F, t26142: F, t4028: F, t22674: F, t28191: F, t80681: F, t1985: F, t22666: F, t28232: F, t26331: F, t26333: F, t90566: F, t28205: F, t7700: F, t90739: F, t28206: F, t6883: F, t1385: F, t1992: F, t22635: F, t3886: F, t6460: F, t6897: F, t12021: F, t1375: F, t16460: F, t20026: F, t26477: F, t5354: F, t6439: F, t6958: F, t6992: F, t7729: F, t80663: F, t80671: F, t90460: F, t90469: F, t90471: F, t90473: F, t90498: F, t90501: F, t22892: F, t28209: F, t22685: F, t6888: F, t6889: F, t6890: F, t12020: F, t225: F, t28051: F, t1386: F, t20044: F, t2016: F, t28187: F, t3758: F, t56640: F, t6993: F, t90525: F, t90534: F, t90542: F, t90547: F, t90550: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t96815, t96818, t96827, t96829, t96830) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2127::<F>(t19451, t6535, t22574, t28830, t31035, t1390, t19631, t1983, t6878, t25989, t91655, t1845, t5356);
        let t96840 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2128::<F>(t26161, t26162, t96830, t26114, t7468, t26179, t1266, t1980, t20098, t27996, t28811, t510, t5450, t650, t652, t671, t6862, t96655, t96796, t96799, t96802, t96805, t96807, t96813, t96815, t96818, t96827, t96829);
        let (t96842, t96844, t96846, t96848, t96851, t96854) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2129::<F>(t26003, t7458, t26142, t4028, t22674, t28191, t80681, t1985, t22666, t28232, t26331, t26333, t90566);
        let (t96857, t96866, t96868, t96873) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2130::<F>(t1985, t22666, t28205, t7700, t90739, t28206, t6883, t1385, t1992, t22635, t3886, t6460);
        let t96885 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2131::<F>(t22674, t28205, t6897, t12021, t1375, t16460, t20026, t26477, t5354, t6439, t6958, t6992, t7729, t80663, t80671, t90460, t90469, t90471, t90473, t90498, t90501, t96848, t96851, t96854, t96857, t96866, t96868, t96873);
        let (t96893, t96896, t96900, t96905, t96910) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2132::<F>(t22674, t22892, t28209, t22666, t22685, t28191, t6888, t19631, t6889, t6890, t12020, t1385, t1992, t22635, t6439);
        let t96917 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2133::<F>(t225, t28051, t1386, t20044, t2016, t28187, t3758, t56640, t6993, t90525, t90534, t90542, t90547, t90550, t96905, t96910);
    (t96840, t96842, t96844, t96846, t96885, t96893, t96896, t96900, t96917)
}
