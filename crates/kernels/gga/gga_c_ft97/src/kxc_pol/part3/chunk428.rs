//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 428/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk428<F: Float>(t3052: F, t378: F, t3051: F, t1639: F, t1640: F, t3042: F, t3045: F, t3048: F) -> (F, F, F) {
    let t3053 = t378 * t3052;
    let t3054 = t3051 * t3053;
    let t3056 = t1639 + t1640 / F::new(9.0) + t3042 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t3045 + F::new(2.0) / F::new(3.0) * t3048 - F::new(2.0) / F::new(3.0) * t3054;
    (t3053, t3054, t3056)
}
