//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 949/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk949<F: Float>(t19977: F, t422: F, t528: F, t61819: F, t929: F, t20612: F, t8959: F, t20603: F, t1554: F, t20596: F, t39942: F, t20859: F, t8392: F) -> (F, F, F, F, F, F, F) {
    let t76914 = t422 * t19977 * t528;
    let t76918 = t61819 * t929;
    let t76926 = t8959 * t20612;
    let t76928 = t8959 * t20603;
    let t76945 = t1554 * t19977;
    let t76982 = F::cast_from(0.22136921132726965153e-3_f64) * t39942 * t20596;
    let t77196 = t8392 * t20859;
    (t76914, t76918, t76926, t76928, t76945, t76982, t77196)
}
