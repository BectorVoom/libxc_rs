//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 581/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk581<F: Float>(t1562: F, t2538: F, t285: F, t3053: F, t3056: F, t3060: F, t3229: F, t499: F, t921: F) -> F {
    let t3232 = t3053 * t285 + t3056 * t285 + t921 * t2538 / F::cast_from(2.0_f64) - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t1562 * t3060 + t499 * t3229 / F::cast_from(4.0_f64);
    t3232
}
