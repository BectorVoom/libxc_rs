//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta258 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1386;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1387;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1388;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1389;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1390;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta258<F: Float>(t3242: F, t460: F, t3247: F, t1176: F, t134: F, t1184: F, t1239: F, t68: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t3576: F, t3604: F, t3585: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t11570 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1386::<F>(t3242, t460);
        let t11583 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1387::<F>(t3247, t460);
        let t11588 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1388::<F>(t1176, t134);
        let (t11589, t11604, t11605, t11606, t11644, t11647, t11649, t11665) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1389::<F>(t11588, t1184, t1239, t68, t1203, t3540, t2393, t374, t486, t485, t3576, t3604);
        let t11668 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1390::<F>(t3585, t820);
    (t11570, t11583, t11588, t11589, t11604, t11605, t11606, t11644, t11647, t11649, t11665, t11668)
}
