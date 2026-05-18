//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 868/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk868<F: Float>(t144: F, t17500: F, t13196: F, t13201: F, t17426: F, t17429: F, t17432: F, t17434: F, t17436: F, t17438: F, t17440: F, t17443: F, t17488: F, t17493: F, t17497: F, t1901: F, t28: F, t446: F, t89: F, t9457: F) -> F {
    let t17501 = t144 * t17500;
    let t17504 = t13196 - F::new(4.0) / F::new(9.0) * t1901 * t17426 + F::new(4.0) / F::new(27.0) * t1901 * t17429 - F::new(2.0) / F::new(27.0) * t17432 + F::new(2.0) / F::new(81.0) * t17434 + t17436 / F::new(27.0) - F::new(2.0) / F::new(9.0) * t17438 + F::new(2.0) / F::new(27.0) * t17440 - t17443 / F::new(9.0) + t89 * t28 * t17488 / F::new(3.0) - F::new(8.0) / F::new(27.0) * t13201 - t9457 - F::new(4.0) / F::new(9.0) * t1901 * t17493 - F::new(2.0) / F::new(9.0) * t1901 * t17497 - t446 * t17501 / F::new(3.0);
    t17504
}
