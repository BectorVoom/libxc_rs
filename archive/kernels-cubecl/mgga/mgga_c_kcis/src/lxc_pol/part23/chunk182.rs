//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 182/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk182<F: Float>(t113: F, t733: F, t738: F, t740: F, t743: F) -> F {
    let t745 = F::cast_from(0.59778596625315888114e-2_f64) * t113 - F::cast_from(0.17565e-2_f64) * t733 + F::cast_from(0.39625e-3_f64) * t738 - F::cast_from(0.1294884726949076719e-4_f64) * t740 + F::cast_from(0.1260328125e-5_f64) * t743;
    t745
}
