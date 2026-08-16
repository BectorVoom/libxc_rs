//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2020/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2020<F: Float>(t103315: F, t103316: F, t103318: F, t103320: F, t103324: F, t106080: F, t106082: F, t106085: F, t106088: F, t106090: F, t93035: F, t95684: F) -> F {
    let t110429 = -t95684 - F::cast_from(0.50820002809285328225e-4_f64) * t106080 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t106082 - t103315 - t103316 + t103318 - t103320 + t103324 + F::cast_from(0.34299214494455789578e-2_f64) * t106085 + F::cast_from(0.54208002996571016773e-3_f64) * t93035 + F::cast_from(0.68598428988911579156e-2_f64) * t106088 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t106090;
    t110429
}
