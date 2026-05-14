//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 729/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk729<F: Float>(t206: F, t8797: F, t8912: F, t209: F, t880: F, t208: F, t214: F, t2733: F, t2742: F, t2748: F, t876: F, t8769: F, t8782: F, t8785: F, t8788: F, t884: F, t888: F) -> (F, F, F, F) {
    let t210 = 0.0 < t206;
    let t8913 = t8797 + t8912;
    let t8915 = piecewise3(t210, t8913, -t8913);
    let t8917 = t209 * t880 * t8915;
    let t8920 = -455.0 / 1296.0 * t8769 * t214 - 35.0 / 144.0 * t2733 * t884 - 7.0 / 48.0 * t876 * t2742 + 7.0 / 96.0 * t876 * t2748 - t208 * t8782 / 16.0 + t8785 * t8788 / 16.0 - t208 * t8917 / 96.0;
    let t8921 = t8920 * t888;
    (t8913, t8915, t8920, t8921)
}
