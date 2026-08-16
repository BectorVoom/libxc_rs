//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1234/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1234<F: Float>(t1279: F, t1281: F, t1851: F, t1853: F, t19023: F, t19037: F, t19041: F, t19044: F, t19047: F, t3403: F, t3407: F, t3410: F, t547: F, t548: F, t5947: F, t5954: F, t5957: F) -> F {
    let t19050 = F::cast_from(12.0_f64) * t1279 * t5954 + F::cast_from(6.0_f64) * t1279 * t5957 + F::cast_from(6.0_f64) * t1281 * t5947 + F::cast_from(6.0_f64) * t1851 * t3407 + F::cast_from(3.0_f64) * t1851 * t3410 + F::cast_from(3.0_f64) * t1853 * t3403 + t19023 * t548 + F::cast_from(6.0_f64) * t19037 * t547 + F::cast_from(12.0_f64) * t19041 * t547 + F::cast_from(6.0_f64) * t19044 * t547 + F::cast_from(3.0_f64) * t19047 * t547;
    t19050
}
