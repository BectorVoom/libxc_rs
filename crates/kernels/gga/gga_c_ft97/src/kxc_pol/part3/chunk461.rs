//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 461/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk461<F: Float>(t3408: F, t526: F, t27: F, t89: F, t1957: F, t1959: F, t1962: F, t3318: F, t3321: F, t3325: F, t3328: F, t3332: F, t3335: F, t3340: F, t3345: F) -> (F, F, F) {
    let t3409 = t526 * t3408;
    let t3411 = t89 * t27 * t3409;
    let t3413 = t1957 + t1959 / F::new(54.0) + t1962 / F::new(18.0) + t3318 / F::new(54.0) - t3321 / F::new(27.0) + t3325 / F::new(18.0) + t3328 / F::new(9.0) - t3332 / F::new(9.0) + t3335 / F::new(18.0) + t3340 / F::new(18.0) + t3345 / F::new(3.0) - t3411 / F::new(6.0);
    (t3409, t3411, t3413)
}
