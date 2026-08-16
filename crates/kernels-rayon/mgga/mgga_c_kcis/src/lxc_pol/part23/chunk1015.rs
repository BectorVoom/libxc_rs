//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1015/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1015(t26508: f64, t7639: f64, t26494: f64, t7636: f64, t26478: f64, t26481: f64, t26483: f64, t26485: f64, t26487: f64, t26491: f64, t26495: f64, t26497: f64, t26502: f64, t26504: f64, t26506: f64) -> f64 {
    let t26509 = t26508 * t7639;
    let t26511 = t7636 * t26494;
    let t26513 = -0.185671721767578125e-4_f64 * t26478 - 0.43285526909722222222e-3_f64 * t26481 - 0.2782641015625e-3_f64 * t26483 - 0.32435763888888888888e-2_f64 * t26485 - 0.32435763888888888888e-2_f64 * t26487 + 0.69505208333333333333e-3_f64 * t26491 + 0.69505208333333333333e-3_f64 * t26495 - 0.13901041666666666667e-2_f64 * t26497 - 0.13901041666666666667e-2_f64 * t26502 + 0.13901041666666666667e-2_f64 * t26504 + 0.13901041666666666667e-2_f64 * t26506 + 0.18550940104166666667e-3_f64 * t26509 + 0.92754700520833333333e-4_f64 * t26511;
    t26513
}
