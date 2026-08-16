//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk889;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk890;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta210(t10481: f64, t10482: f64, t1021: f64, t248: f64, t2776: f64, t3051: f64, t1041: f64, t10316: f64, t1044: f64, t3103: f64, t3109: f64, t10309: f64, t3062: f64, t3114: f64, t376: f64, t676: f64, t1023: f64, t1020: f64, t10433: f64, t10436: f64, t10438: f64, t10441: f64, t10446: f64, t10449: f64, t10455: f64, t10460: f64, t10463: f64, t10480: f64, t3039: f64, t3048: f64, t3064: f64, t3098: f64, t3117: f64, t3123: f64, t378: f64, t1017: f64, t3087: f64, t1015: f64, t1012: f64, t2940: f64, t2952: f64, t2928: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10483, t10485, t10489, t10490, t10493, t10496, t10501) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk889(t10481, t10482, t1021, t248, t2776, t3051, t1041, t10316, t1044, t3103, t3109, t10309, t3062);
        let (t10508, t10510, t10513) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk890(t3103, t3114, t376, t676, t1023, t248, t1020, t1041, t10433, t10436, t10438, t10441, t10446, t10449, t10455, t10460, t10463, t10480, t10485, t10490, t10493, t10496, t10501, t3039, t3048, t3064, t3098, t3117, t3123, t378);
        let (t10516, t10517, t10521, t10523) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk891(t1017, t3087, t1015, t1012, t2940, t2952, t2928, t320);
    (t10483, t10485, t10489, t10493, t10501, t10508, t10510, t10513, t10516, t10517, t10521, t10523)
}
