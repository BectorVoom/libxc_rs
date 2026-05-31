//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 89/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk89<F: Float>(t141: F, t165: F, zeta_threshold: F) -> (F, F, F) {
    let t225 = F::cast_from(2.0_f64) <= zeta_threshold;
    let t228 = F::cast_from(0.0_f64) <= zeta_threshold;
    let t280 = piecewise3::<F>(t225, t141, t165);
    let t281 = piecewise3::<F>(t228, t141, F::cast_from(0.0_f64));
    let t283 = t280 / F::cast_from(2.0_f64) + t281 / F::cast_from(2.0_f64);
    let t284 = t283 * t283;
    let t286 = F::cast_from(1.0_f64) / t284 / t283;
    (t283, t284, t286)
}
