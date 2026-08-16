//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 353/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk353<F: Float>(t24: F, t1263: F, t1265: F, t422: F, t423: F, t330: F, t148: F, t95: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t1269 = piecewise3::<F>(t332, F::cast_from(0.0_f64), t1263 * t423 / F::cast_from(2.0_f64) + t422 * t1265 / F::cast_from(2.0_f64));
    let t1270 = t330 * t1269;
    let t1281 = t148 * t95;
    (t1270, t1281)
}
