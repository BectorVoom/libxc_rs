//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk691;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk692;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk693;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk694;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta131<F: Float>(t1216: F, t248: F, t3570: F, t1213: F, t478: F, t483: F, t3068: F, t1244: F, t1230: F, t820: F, t1090: F, t1089: F, t415: F, t61: F, t3243: F, t1174: F, t1218: F, t1227: F, t1232: F, t3490: F, t3496: F, t3506: F, t3511: F, t3515: F, t3518: F, t3524: F, t3527: F, t3531: F, t3536: F, t3542: F, t3543: F, t3547: F, t3549: F, t3552: F, t3557: F, t3562: F, t3567: F, t488: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3572, t3573, t3575, t3576, t3577) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk691::<F>(t1216, t248, t3570, t1213, t478, t483, t3068, t1244);
        let t3578 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk692::<F>(t1230, t820);
        let (t3579, t3580, t3584) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk693::<F>(t1090, t1216, t3578, t1089, t415);
        let (t3585, t3587, t3590) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk694::<F>(t3584, t61, t248, t3243, t1174, t1213, t1218, t1227, t1232, t3490, t3496, t3506, t3511, t3515, t3518, t3524, t3527, t3531, t3536, t3542, t3543, t3547, t3549, t3552, t3557, t3562, t3567, t3573, t3577, t3580, t488);
    (t3572, t3575, t3576, t3577, t3578, t3579, t3580, t3584, t3585, t3587, t3590)
}
