//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1205/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1205<F: Float>(t20151: F, t5078: F, t19945: F, t19948: F, t19951: F, t19954: F, t19958: F, t19961: F, t19963: F, t19965: F, t19967: F, t19970: F, t20131: F, t20134: F, t20137: F, t20139: F, t20143: F, t20146: F, t20149: F) -> (F, F) {
    let t20152 = t20151 * t5078;
    let t20154 = t19945 / F::new(96.0) + t19948 / F::new(864.0) + t19951 / F::new(12.0) + t19954 / F::new(8.0) + t19958 / F::new(24.0) - t19961 / F::new(64.0) + t19963 / F::new(128.0) - t19965 / F::new(72.0) - t19967 / F::new(96.0) - t19970 / F::new(9.0) + t20131 / F::new(16.0) - t20134 / F::new(3.0) + t20137 / F::new(4.0) - F::new(2.0) / F::new(9.0) * t20139 - t20143 / F::new(16.0) + t20146 / F::new(256.0) + t20149 / F::new(6.0) - t20152 / F::new(36.0);
    (t20152, t20154)
}
