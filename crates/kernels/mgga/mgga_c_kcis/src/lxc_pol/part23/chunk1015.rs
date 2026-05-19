//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1015/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1015<F: Float>(t26508: F, t7639: F, t26494: F, t7636: F, t26478: F, t26481: F, t26483: F, t26485: F, t26487: F, t26491: F, t26495: F, t26497: F, t26502: F, t26504: F, t26506: F) -> F {
    let t26509 = t26508 * t7639;
    let t26511 = t7636 * t26494;
    let t26513 = -F::cast_from(0.185671721767578125e-4_f64) * t26478 - F::cast_from(0.43285526909722222222e-3_f64) * t26481 - F::cast_from(0.2782641015625e-3_f64) * t26483 - F::cast_from(0.32435763888888888888e-2_f64) * t26485 - F::cast_from(0.32435763888888888888e-2_f64) * t26487 + F::cast_from(0.69505208333333333333e-3_f64) * t26491 + F::cast_from(0.69505208333333333333e-3_f64) * t26495 - F::cast_from(0.13901041666666666667e-2_f64) * t26497 - F::cast_from(0.13901041666666666667e-2_f64) * t26502 + F::cast_from(0.13901041666666666667e-2_f64) * t26504 + F::cast_from(0.13901041666666666667e-2_f64) * t26506 + F::cast_from(0.18550940104166666667e-3_f64) * t26509 + F::cast_from(0.92754700520833333333e-4_f64) * t26511;
    t26513
}
