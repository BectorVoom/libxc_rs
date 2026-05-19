//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 304/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk304<F: Float>(t120: F, t3363: F, t383: F, t929: F, t72: F, t1005: F, t126: F, t3056: F, t1631: F, t2014: F, t2021: F, t3359: F, t3360: F, t534: F) -> F {
    let t3364 = t3363 * t120;
    let t3366 = t929 * t383;
    let t3368 = t72 * t3366 * t120;
    let t3371 = t1005 * t383;
    let t3374 = t3056 * t126;
    let t3379 = -F::cast_from(0.11705142615505742e0_f64) * t3359 * t3360 + F::cast_from(0.23410285231011484e0_f64) * t3364 - F::cast_from(0.26564305359272358183e-2_f64) * t2014 * t3368 + F::cast_from(0.319782988780431561e-1_f64) * t2021 * t3371 - F::cast_from(0.532971647967385935e-1_f64) * t534 * t3374 + F::cast_from(0.13977476158628290272e-1_f64) * t1631 * t3371;
    t3379
}
