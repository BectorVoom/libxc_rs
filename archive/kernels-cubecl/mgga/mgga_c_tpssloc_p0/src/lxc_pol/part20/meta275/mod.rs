//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1444;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1445;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta275<F: Float>(t10481: F, t10482: F, t1021: F, t248: F, t2776: F, t3051: F, t1041: F, t10316: F, t1044: F, t3103: F, t3109: F, t10309: F, t3062: F, t3114: F, t376: F, t676: F, t1023: F, t1020: F, t10433: F, t10436: F, t10438: F, t10441: F, t10446: F, t10449: F, t10455: F, t10460: F, t10463: F, t10480: F, t3039: F, t3048: F, t3064: F, t3098: F, t3117: F, t3123: F, t378: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10483, t10485, t10489, t10490, t10493, t10496, t10501) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1444::<F>(t10481, t10482, t1021, t248, t2776, t3051, t1041, t10316, t1044, t3103, t3109, t10309, t3062);
        let (t10504, t10508) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1445::<F>(t3103, t3114, t376, t676);
        let (t10510, t10511, t10513) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1446::<F>(t1023, t10508, t248, t1020, t1041, t10433, t10436, t10438, t10441, t10446, t10449, t10455, t10460, t10463, t10480, t10485, t10490, t10493, t10496, t10501, t10504, t3039, t3048, t3064, t3098, t3114, t3117, t3123, t378);
    (t10483, t10485, t10489, t10490, t10493, t10496, t10501, t10504, t10508, t10510, t10511, t10513)
}
