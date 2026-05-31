//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 950/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk950<F: Float>(t20151: F, t5078: F, t19945: F, t19948: F, t19951: F, t19954: F, t19958: F, t19961: F, t19963: F, t19965: F, t19967: F, t19970: F, t20131: F, t20134: F, t20137: F, t20139: F, t20143: F, t20146: F, t20149: F) -> (F, F) {
    let t20152 = t20151 * t5078;
    let t20154 = t19945 / F::cast_from(96.0_f64) + t19948 / F::cast_from(864.0_f64) + t19951 / F::cast_from(12.0_f64) + t19954 / F::cast_from(8.0_f64) + t19958 / F::cast_from(24.0_f64) - t19961 / F::cast_from(64.0_f64) + t19963 / F::cast_from(128.0_f64) - t19965 / F::cast_from(72.0_f64) - t19967 / F::cast_from(96.0_f64) - t19970 / F::cast_from(9.0_f64) + t20131 / F::cast_from(16.0_f64) - t20134 / F::cast_from(3.0_f64) + t20137 / F::cast_from(4.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t20139 - t20143 / F::cast_from(16.0_f64) + t20146 / F::cast_from(256.0_f64) + t20149 / F::cast_from(6.0_f64) - t20152 / F::cast_from(36.0_f64);
    (t20152, t20154)
}
