//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 880/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk880<F: Float>(t584: F, t7257: F, t583: F, t1546: F, t7276: F, t7278: F, t7280: F, t7284: F, t7288: F, t7290: F, t7292: F, t7294: F, t7297: F, t7300: F, t7302: F) -> (F, F, F) {
    let t7304 = t584 * t7257;
    let t7305 = t583 * t7304;
    let t7306 = t1546 * t7305;
    let t7308 = -t7276 / F::cast_from(576.0_f64) - t7278 / F::cast_from(3.0_f64) + t7280 / F::cast_from(12.0_f64) - t7284 / F::cast_from(16.0_f64) - t7288 / F::cast_from(192.0_f64) + t7290 / F::cast_from(24.0_f64) - t7292 / F::cast_from(96.0_f64) + t7294 / F::cast_from(128.0_f64) - t7297 / F::cast_from(24.0_f64) + t7300 / F::cast_from(96.0_f64) - t7302 / F::cast_from(8.0_f64) + t7306 / F::cast_from(256.0_f64);
    (t7305, t7306, t7308)
}
