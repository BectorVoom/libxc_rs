//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 773/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk773<F: Float>(t254: F, t542: F, t252: F, t245: F, t713: F, t1697: F, t212: F, t22: F, t1923: F, t707: F, t256: F, t1914: F, t1918: F) -> (F, F, F, F, F) {
    let t5385 = t254 * t542;
    let t5387 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t252 * t5385;
    let t5390 = t245 * t713;
    let t5399 = F::cast_from(1.0_f64) / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5416 = t707 * t1923;
    let t5417 = t5416 * t256;
    let t5418 = t1914 * t1918;
    (t5387, t5390, t5400, t5417, t5418)
}
