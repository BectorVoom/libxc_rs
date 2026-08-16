//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 103/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk103<F: Float>(t340: F, t379: F, t382: F, sigma0: F) -> (F, F) {
    let t385 = F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t340 * t379 * t382;
    let t386 = t385 < -F::cast_from(0.66725e-1_f64);
    let t388 = piecewise3::<F>(t386, F::cast_from(0.0_f64), F::cast_from(0.66725e-1_f64) + t385);
    let t389 = t388 * sigma0;
    (t389, t385)
}
