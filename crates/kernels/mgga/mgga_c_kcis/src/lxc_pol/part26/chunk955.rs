//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 955/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk955<F: Float>(t2155: F, t26494: F, t26477: F, t209: F, t2739: F, t2740: F, t888: F, t7633: F, t7647: F, t7639: F, t695: F, t8759: F, t7636: F, t26478: F, t26481: F, t26483: F, t26485: F, t26487: F, t26491: F) -> (F, F, F) {
    let t26495 = t2155 * t26494;
    let t26497 = t2155 * t26477;
    let t26501 = t209 * t2739 * t888 * t2740;
    let t26502 = t2155 * t26501;
    let t26504 = t7633 * t7647;
    let t26506 = t7633 * t7639;
    let t26508 = t8759 * t695;
    let t26509 = t26508 * t7639;
    let t26511 = t7636 * t26494;
    let t26513 = -0.185671721767578125e-4 * t26478 - 0.43285526909722222222e-3 * t26481 - 0.2782641015625e-3 * t26483 - 0.32435763888888888888e-2 * t26485 - 0.32435763888888888888e-2 * t26487 + 0.69505208333333333333e-3 * t26491 + 0.69505208333333333333e-3 * t26495 - 0.13901041666666666667e-2 * t26497 - 0.13901041666666666667e-2 * t26502 + 0.13901041666666666667e-2 * t26504 + 0.13901041666666666667e-2 * t26506 + 0.18550940104166666667e-3 * t26509 + 0.92754700520833333333e-4 * t26511;
    (t26501, t26508, t26513)
}
