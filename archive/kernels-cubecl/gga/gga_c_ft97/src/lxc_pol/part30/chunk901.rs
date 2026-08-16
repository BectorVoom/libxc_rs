//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 901/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk901<F: Float>(t2404: F, t2680: F, t683: F, t7640: F, t191: F, t33828: F, t190: F, t305: F, t36452: F, t37991: F, t10362: F, t289: F) -> (F, F, F, F, F) {
    let t43350 = t2404 * t2680;
    let t43381 = t683 * t7640;
    let t43524 = t191 * t33828;
    let t43548 = F::cast_from(1.0_f64) / t305 / t37991 / t190 / t2680 / t36452 / F::cast_from(96.0_f64);
    let t43585 = F::cast_from(1.0_f64) / t10362 / t289;
    (t43350, t43381, t43524, t43548, t43585)
}
