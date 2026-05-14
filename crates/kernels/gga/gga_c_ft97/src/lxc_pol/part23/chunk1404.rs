//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1404/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1404<F: Float>(t127836: F, t127839: F, t127842: F, t127846: F, t127848: F, t127852: F, t127855: F, t127858: F, t127861: F, t127866: F, t127869: F, t127872: F, t114482: F, t114483: F, t127877: F, t127879: F, t127882: F, t127887: F, t127892: F, t127894: F, t127898: F, t127903: F, t127908: F, t127912: F) -> (F, F) {
    let t128319 = -2.0 / 9.0 * t127836 + 5.0 / 81.0 * t127839 - 4.0 / 27.0 * t127842 + t127846 / 2.0 + t127848 / 54.0 - t127852 / 18.0 - t127855 / 9.0 - t127858 / 9.0 + t127861 / 27.0 - t127866 / 54.0 - 4.0 / 9.0 * t127869 + 4.0 / 27.0 * t127872;
    let t128330 = t127877 / 3.0 - 2.0 / 9.0 * t127879 + 2.0 / 27.0 * t127882 + t114482 + t114483 + t127887 / 6.0 + t127892 / 3.0 - 4.0 / 9.0 * t127894 + t127898 / 18.0 - t127903 / 8.0 - t127908 / 6.0 - 2.0 / 9.0 * t127912;
    (t128319, t128330)
}
