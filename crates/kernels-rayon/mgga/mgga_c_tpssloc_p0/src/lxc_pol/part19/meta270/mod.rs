//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1026;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1028;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta270(t11918: f64, t1241: f64, t11868: f64, t466: f64, t225: f64, t3591: f64, t3482: f64, t1190: f64, t3590: f64, t1251: f64, t3630: f64, t3598: f64, t11599: f64, t11601: f64, t11608: f64, t11613: f64, t1238: f64, t1252: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t498: f64, t1254: f64, t3637: f64, t3639: f64, t500: f64, t11405: f64, t11409: f64, t11426: f64, t11429: f64, t11472: f64, t11480: f64, t11482: f64, t11484: f64, t11631: f64, t11636: f64, t1256: f64, t193: f64, t336: f64, t3633: f64, t3640: f64, t4700: f64, t28: f64, t265: f64, t504: f64, t10150: f64, t11476: f64, t1081: f64, t11122: f64, t1260: f64, t2250: f64, t2756: f64, t3231: f64, t3644: f64, t506: f64, t52: f64, t607: f64, t873: f64, t9258: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11919, t11923, t11925, t11928, t11931, t11935) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1026(t11918, t1241, t11868, t466, t225, t3591, t3482, t1190, t3590, t1251, t3630, t3598);
        let t11940 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027(t11599, t11601, t11608, t11613, t11919, t11923, t11925, t11928, t11931, t11935, t1238, t1252, t3487, t3593, t3600, t3631, t498);
        let (t11947, t11955) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1028(t1254, t3637, t3639, t500, t11405, t11409, t11426, t11429, t11472, t11480, t11482, t11484, t11631, t11636, t11940, t1256, t193, t336, t3633, t3640, t4700);
        let (t11957, t11967) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1029(t28, t265, t504, t10150, t11476, t11955, t1081, t11122, t1260, t2250, t2756, t3231, t3644, t506, t52, t607, t873, t9258, dens_threshold, rho1, zeta_threshold);
    (t11919, t11923, t11925, t11928, t11931, t11935, t11940, t11947, t11957, t11967)
}
