//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 638/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk638<F: Float>(t7276: F, t7278: F, t7280: F, t7284: F, t7288: F, t7290: F, t7292: F, t7294: F, t7297: F, t7300: F, t7302: F, t7306: F, t7311: F, t7315: F, t7319: F, t7323: F, t7330: F, t7333: F, t7336: F, t7339: F, t7383: F, t7387: F, t7390: F, t7394: F) -> (F, F) {
    let t7552 = -0.44965277777777777777e-2 * t7276 - 0.5e0 * t7278 + 0.125e0 * t7280 - 0.9375e-1 * t7284 - 0.13489583333333333333e-1 * t7288 + 0.10791666666666666667e0 * t7290 - 0.26979166666666666666e-1 * t7292 + 0.20234375e-1 * t7294 - 0.10791666666666666667e0 * t7297 + 0.26979166666666666666e-1 * t7300 - 0.1875e0 * t7302 + 0.101171875e-1 * t7306;
    let t7565 = -0.20833333333333333333e-1 * t7311 + 0.625e-1 * t7315 - 0.20234375e-1 * t7319 - 0.101171875e-1 * t7323 - 0.34173611111111111111e0 * t7330 + 0.14388888888888888889e0 * t7333 + 0.5e0 * t7336 - 0.125e0 * t7339 + 0.9375e-1 * t7383 + 0.91666666666666666667e0 * t7387 - 0.33333333333333333334e0 * t7390 + 0.1875e0 * t7394;
    (t7552, t7565)
}
