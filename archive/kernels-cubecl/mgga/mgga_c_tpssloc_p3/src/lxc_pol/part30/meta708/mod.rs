//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2336;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2337;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2338;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2339;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta708<F: Float>(t28017: F, t3941: F, t671: F, t20173: F, t28899: F, t1395: F, t5456: F, t1873: F, t20162: F, t6534: F, t26545: F, t33185: F, t12524: F, t28896: F, t5493: F, t100902: F, t100908: F, t100911: F, t100915: F, t100917: F, t100921: F, t100924: F, t1458: F, t19534: F, t20181: F, t23880: F, t5376: F, t7010: F, t86647: F, t86656: F, t28904: F, t576: F, t28868: F, t580: F, t100900: F, t1398: F, t1404: F, t1858: F, t20149: F, t20186: F, t2023: F, t2029: F, t26510: F, t28869: F, t5364: F, t6471: F, t7020: F, t7774: F, t86565: F, t86567: F, t86571: F, t96348: F, t7758: F, t6470: F, t1851: F, t100867: F, t1396: F, t1852: F, t26555: F, t3: F, t5381: F, t6483: F, t7003: F, t7759: F, t86579: F, t91813: F, t91816: F, t91818: F, t91824: F) -> F {
        let (t100927, t100929, t100932, t100934, t100936) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2336::<F>(t28017, t3941, t671, t20173, t28899, t1395, t5456, t1873, t20162, t6534, t26545, t33185);
        let t100942 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2337::<F>(t12524, t28896, t3941, t5493, t6534, t100902, t100908, t100911, t100915, t100917, t100921, t100924, t100927, t100929, t100932, t100934, t100936, t1458, t19534, t20181, t23880, t5376, t671, t7010, t86647, t86656);
        let t100948 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2338::<F>(t28904, t576, t28868, t580, t100900, t100942, t1398, t1404, t1858, t20149, t20186, t2023, t2029, t26510, t28869, t5364, t6471, t7020, t7774, t86565, t86567, t86571, t96348);
        let t100962 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2339::<F>(t1858, t7758, t2029, t6470, t1851, t7774, t100867, t1396, t1852, t26555, t28904, t3, t5381, t580, t6483, t7003, t7759, t86579, t91813, t91816, t91818, t91824);
        let tv4rho3sigma6 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2340::<F>(t100948, t100962);
    tv4rho3sigma6
}
