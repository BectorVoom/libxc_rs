//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1026;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1028;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta270<F: Float>(t11918: F, t1241: F, t11868: F, t466: F, t225: F, t3591: F, t3482: F, t1190: F, t3590: F, t1251: F, t3630: F, t3598: F, t11599: F, t11601: F, t11608: F, t11613: F, t1238: F, t1252: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F, t1254: F, t3637: F, t3639: F, t500: F, t11405: F, t11409: F, t11426: F, t11429: F, t11472: F, t11480: F, t11482: F, t11484: F, t11631: F, t11636: F, t1256: F, t193: F, t336: F, t3633: F, t3640: F, t4700: F, t28: F, t265: F, t504: F, t10150: F, t11476: F, t1081: F, t11122: F, t1260: F, t2250: F, t2756: F, t3231: F, t3644: F, t506: F, t52: F, t607: F, t873: F, t9258: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11919, t11923, t11925, t11928, t11931, t11935) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1026::<F>(t11918, t1241, t11868, t466, t225, t3591, t3482, t1190, t3590, t1251, t3630, t3598);
        let t11940 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027::<F>(t11599, t11601, t11608, t11613, t11919, t11923, t11925, t11928, t11931, t11935, t1238, t1252, t3487, t3593, t3600, t3631, t498);
        let (t11947, t11955) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1028::<F>(t1254, t3637, t3639, t500, t11405, t11409, t11426, t11429, t11472, t11480, t11482, t11484, t11631, t11636, t11940, t1256, t193, t336, t3633, t3640, t4700);
        let (t11957, t11967) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1029::<F>(t28, t265, t504, t10150, t11476, t11955, t1081, t11122, t1260, t2250, t2756, t3231, t3644, t506, t52, t607, t873, t9258, dens_threshold, rho1, zeta_threshold);
    (t11919, t11923, t11925, t11928, t11931, t11935, t11940, t11947, t11957, t11967)
}
