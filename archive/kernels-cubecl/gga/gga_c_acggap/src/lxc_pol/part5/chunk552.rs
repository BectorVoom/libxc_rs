//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 552/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk552<F: Float>(t121: F, t3151: F, t126: F, t147: F, t3036: F, t383: F) -> (F, F, F, F) {
    let t3206 = t121 * t3151;
    let t3207 = t3206 * t126;
    let t3209 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t3207 * t147;
    let t3210 = t3036 * t383;
    (t3206, t3207, t3209, t3210)
}
