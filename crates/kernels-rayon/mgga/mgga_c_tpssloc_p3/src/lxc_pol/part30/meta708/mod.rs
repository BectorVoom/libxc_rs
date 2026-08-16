//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2336;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2337;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2338;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2339;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta708(t28017: f64, t3941: f64, t671: f64, t20173: f64, t28899: f64, t1395: f64, t5456: f64, t1873: f64, t20162: f64, t6534: f64, t26545: f64, t33185: f64, t12524: f64, t28896: f64, t5493: f64, t100902: f64, t100908: f64, t100911: f64, t100915: f64, t100917: f64, t100921: f64, t100924: f64, t1458: f64, t19534: f64, t20181: f64, t23880: f64, t5376: f64, t7010: f64, t86647: f64, t86656: f64, t28904: f64, t576: f64, t28868: f64, t580: f64, t100900: f64, t1398: f64, t1404: f64, t1858: f64, t20149: f64, t20186: f64, t2023: f64, t2029: f64, t26510: f64, t28869: f64, t5364: f64, t6471: f64, t7020: f64, t7774: f64, t86565: f64, t86567: f64, t86571: f64, t96348: f64, t7758: f64, t6470: f64, t1851: f64, t100867: f64, t1396: f64, t1852: f64, t26555: f64, t3: f64, t5381: f64, t6483: f64, t7003: f64, t7759: f64, t86579: f64, t91813: f64, t91816: f64, t91818: f64, t91824: f64) -> f64 {
        let (t100927, t100929, t100932, t100934, t100936) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2336(t28017, t3941, t671, t20173, t28899, t1395, t5456, t1873, t20162, t6534, t26545, t33185);
        let t100942 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2337(t12524, t28896, t3941, t5493, t6534, t100902, t100908, t100911, t100915, t100917, t100921, t100924, t100927, t100929, t100932, t100934, t100936, t1458, t19534, t20181, t23880, t5376, t671, t7010, t86647, t86656);
        let t100948 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2338(t28904, t576, t28868, t580, t100900, t100942, t1398, t1404, t1858, t20149, t20186, t2023, t2029, t26510, t28869, t5364, t6471, t7020, t7774, t86565, t86567, t86571, t96348);
        let t100962 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2339(t1858, t7758, t2029, t6470, t1851, t7774, t100867, t1396, t1852, t26555, t28904, t3, t5381, t580, t6483, t7003, t7759, t86579, t91813, t91816, t91818, t91824);
        let tv4rho3sigma6 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2340(t100948, t100962);
    tv4rho3sigma6
}
