//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 645/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk645<F: Float>(t301: F, t5069: F, t1403: F, t839: F, t402: F, t4099: F, t1396: F, t1402: F, t1404: F, t1407: F, t153: F, t155: F, t400: F, t403: F, t5050: F, t5060: F, t5066: F, t519: F, t521: F, t917: F, t923: F, t926: F) -> F {
    let t5070 = t5069 * t301;
    let t5073 = t1403 * t839;
    let t5076 = t402 * t4099;
    let t5079 = F::cast_from(6.0_f64) * t1396 * t403 + F::cast_from(60.0_f64) * t1402 * t5066 - F::cast_from(24.0_f64) * t1402 * t5070 - F::cast_from(12.0_f64) * t1402 * t5073 - F::cast_from(24.0_f64) * t1404 * t5060 + F::cast_from(6.0_f64) * t1407 * t400 + F::cast_from(3.0_f64) * t153 * t5076 - t155 * t5050 - F::cast_from(12.0_f64) * t519 * t923 + F::cast_from(3.0_f64) * t519 * t926 + F::cast_from(3.0_f64) * t521 * t917;
    t5079
}
