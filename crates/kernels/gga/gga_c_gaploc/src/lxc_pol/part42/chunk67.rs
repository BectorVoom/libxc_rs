//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 67/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk67<F: Float>(t141: F, t165: F, zeta_threshold: F) -> (F, F, F) {
    let t225 = F::new(2.0) <= zeta_threshold;
    let t228 = F::new(0.0) <= zeta_threshold;
    let t280 = piecewise3::<F>(t225, t141, t165);
    let t281 = piecewise3::<F>(t228, t141, F::new(0.0));
    let t283 = t280 / F::new(2.0) + t281 / F::new(2.0);
    let t284 = t283 * t283;
    let t286 = F::new(1.0) / t284 / t283;
    (t283, t284, t286)
}
