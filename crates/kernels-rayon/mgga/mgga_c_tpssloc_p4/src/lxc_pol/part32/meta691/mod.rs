//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta691 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2137;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2138;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2139;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2140;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta691(t26179: f64, t7468: f64, t26003: f64, t7458: f64, t26142: f64, t4028: f64, t22674: f64, t28191: f64, t80681: f64, t1985: f64, t22666: f64, t28232: f64, t26331: f64, t26333: f64, t90566: f64, t28205: f64, t7700: f64, t90739: f64, t28206: f64, t6883: f64, t1385: f64, t1992: f64, t22635: f64, t3886: f64, t6460: f64, t6897: f64, t12021: f64, t1375: f64, t16460: f64, t20026: f64, t26477: f64, t5354: f64, t6439: f64, t6958: f64, t6992: f64, t7729: f64, t80663: f64, t80671: f64, t90460: f64, t90469: f64, t90471: f64, t90473: f64, t90498: f64, t90501: f64, t22892: f64, t28209: f64, t22685: f64, t6888: f64, t19631: f64, t6889: f64, t6890: f64, t12020: f64, t225: f64, t28051: f64, t1386: f64, t20044: f64, t2016: f64, t28187: f64, t3758: f64, t56640: f64, t6993: f64, t90525: f64, t90534: f64, t90542: f64, t90547: f64, t90550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96839, t96842, t96844, t96846, t96848, t96851) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2137(t26179, t7468, t26003, t7458, t26142, t4028, t22674, t28191, t80681, t1985, t22666, t28232);
        let (t96854, t96857, t96866, t96868, t96873) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2138(t26331, t26333, t90566, t1985, t22666, t28205, t7700, t90739, t28206, t6883, t1385, t1992, t22635, t3886, t6460);
        let t96885 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2139(t22674, t28205, t6897, t12021, t1375, t16460, t20026, t26477, t5354, t6439, t6958, t6992, t7729, t80663, t80671, t90460, t90469, t90471, t90473, t90498, t90501, t96848, t96851, t96854, t96857, t96866, t96868, t96873);
        let (t96893, t96896, t96900, t96905, t96910) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2140(t22674, t22892, t28209, t22666, t22685, t28191, t6888, t19631, t6889, t6890, t12020, t1385, t1992, t22635, t6439);
        let t96917 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2141(t225, t28051, t1386, t20044, t2016, t28187, t3758, t56640, t6993, t90525, t90534, t90542, t90547, t90550, t96905, t96910);
    (t96839, t96842, t96844, t96846, t96885, t96893, t96896, t96900, t96917)
}
