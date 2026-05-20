//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3703/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3703<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F) -> F {
    let t70172 = F::cast_from(0.59266666666666666668e-1_f64) * t68297 + F::cast_from(0.29633333333333333334e-1_f64) * t68301 + F::cast_from(0.88900000000000000002e-1_f64) * t68305 - F::cast_from(0.43901234567901234568e-1_f64) * t68310 + F::cast_from(0.65851851851851851854e-2_f64) * t68332 + F::cast_from(0.13170370370370370371e-1_f64) * t68334 + F::cast_from(0.39511111111111111112e-1_f64) * t68336 + F::cast_from(0.16462962962962962963e-1_f64) * t68342 + F::cast_from(0.19755555555555555556e0_f64) * t68347 - F::cast_from(0.59266666666666666668e-1_f64) * t68350 - F::cast_from(0.35560000000000000001e0_f64) * t68353 - F::cast_from(0.19755555555555555556e-1_f64) * t68357 + F::new(0.3556e0) * t68360;
    t70172
}
