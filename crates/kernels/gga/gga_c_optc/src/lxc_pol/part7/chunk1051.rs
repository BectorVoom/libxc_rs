//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1051/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1051<F: Float>(t23682: F, t23620: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23660: F, t23859: F, t787: F, t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23840: F, t23842: F, t23846: F) -> (F, F, F) {
    let t23860 = 280.0 / 81.0 * t23682;
    let t23872 = t23860 - 8.0 / 9.0 * t23620 - 16.0 / 27.0 * t23622 + 4.0 / 9.0 * t23624 + 40.0 / 81.0 * t23626 - 80.0 / 81.0 * t23630 - t23633 / 3.0 + 112.0 / 81.0 * t23635 - 16.0 / 9.0 * t23637 + 40.0 / 9.0 * t23640 + 2.0 * t23644 + 8.0 / 3.0 * t23660;
    let t23873 = t23859 + t23872;
    let t23874 = t787 * t23873;
    let t23882 = -0.18396666666666666667e0 * t23543 - 0.44152e0 * t23545 + 0.44152e0 * t23551 + 0.98115555555555555556e0 * t23553 + 0.247573125e0 * t23840 - 0.3883875e1 * t23842 + 0.6189328125e-1 * t23846 + 0.16504875e0 * t23874 + 0.11038e1 * t23555 + 0.132456e1 * t23557 - 0.99342e0 * t23561 - 0.82785e-1 * t23565 + 0.22076e0 * t23567 + 0.98115555555555555555e-1 * t23569;
    (t23873, t23874, t23882)
}
