//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 940/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk940<F: Float>(t1134: F, t3206: F, t3703: F, t858: F, t13265: F, t6402: F, t13557: F, t2289: F, t13126: F, t21597: F, t3123: F, t38063: F, t13249: F, t37138: F, t12041: F, t36666: F) -> (F, F, F, F, F, F, F, F) {
    let t45660 = t3206 * t858 * t1134 * t3703;
    let t45703 = t6402 * t13265;
    let t45741 = t2289 * t13557;
    let t45750 = t13126 * t21597;
    let t45753 = t3123 * t38063;
    let t45755 = t6402 * t13249;
    let t45767 = t3123 * t37138;
    let t45771 = t12041 * t36666;
    (t45660, t45703, t45741, t45750, t45753, t45755, t45767, t45771)
}
