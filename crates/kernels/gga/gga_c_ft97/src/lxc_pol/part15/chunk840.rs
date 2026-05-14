//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 840/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk840<F: Float>(t61819: F, t929: F, t20612: F, t8959: F, t20603: F, t1554: F, t19977: F, t20596: F, t39942: F, t20859: F, t8392: F, t20875: F, t20765: F, t1882: F, t20737: F, t20904: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76918 = t61819 * t929;
    let t76926 = t8959 * t20612;
    let t76928 = t8959 * t20603;
    let t76945 = t1554 * t19977;
    let t76982 = 0.22136921132726965153e-3 * t39942 * t20596;
    let t77196 = t8392 * t20859;
    let t77198 = t8392 * t20875;
    let t77214 = t8392 * t20765;
    let t77305 = t1882 * t20737;
    let t77307 = t1882 * t20904;
    (t76918, t76926, t76928, t76945, t76982, t77196, t77198, t77214, t77305, t77307)
}
