//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 759/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk759<F: Float>(t12703: F, t16666: F, t3455: F, t925: F, t9144: F, t2142: F, t4805: F, t144: F, t13196: F, t13201: F, t17426: F, t17429: F, t17432: F, t17434: F, t17436: F, t17438: F, t17440: F, t17443: F, t17488: F, t1901: F, t28: F, t446: F, t89: F, t9457: F) -> (F, F) {
    let t17493 = t12703 * t16666;
    let t17496 = t925 * t3455;
    let t17497 = t9144 * t17496;
    let t17500 = t2142 * t4805;
    let t17501 = t144 * t17500;
    let t17504 = t13196 - 4.0 / 9.0 * t1901 * t17426 + 4.0 / 27.0 * t1901 * t17429 - 2.0 / 27.0 * t17432 + 2.0 / 81.0 * t17434 + t17436 / 27.0 - 2.0 / 9.0 * t17438 + 2.0 / 27.0 * t17440 - t17443 / 9.0 + t89 * t28 * t17488 / 3.0 - 8.0 / 27.0 * t13201 - t9457 - 4.0 / 9.0 * t1901 * t17493 - 2.0 / 9.0 * t1901 * t17497 - t446 * t17501 / 3.0;
    (t17500, t17504)
}
