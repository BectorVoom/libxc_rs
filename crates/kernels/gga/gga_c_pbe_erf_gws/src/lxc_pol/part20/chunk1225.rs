//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1225/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1225<F: Float>(t1176: F, t2298: F, t923: F, t51649: F, t867: F, t3966: F, t326: F, t378: F, t6594: F, t745: F, t837: F, t2306: F, t938: F) -> (F, F, F, F, F, F) {
    let t51963 = t1176 * t923 * t2298;
    let t51966 = t51649 * t867;
    let t51967 = t51966 * t3966;
    let t51977 = t326 * t6594 * t378;
    let t51978 = F::new(455.0) / F::new(1296.0) * t51977;
    let t51989 = t745 * t837;
    let t52000 = t2306 * t938;
    (t51963, t51966, t51967, t51978, t51989, t52000)
}
