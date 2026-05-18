//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1050/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1050<F: Float>(t27479: F, t303: F, t2244: F, t3245: F, t110: F, t2238: F, t2237: F, t27342: F, t27416: F, t27455: F, t27459: F, t27462: F, t27465: F, t27471: F, t27477: F, t7898: F, t7908: F, t7911: F) -> (F, F, F, F, F, F) {
    let t27480 = t303 * t27479;
    let t27482 = t3245 * t2244;
    let t27483 = F::new(0.55273148148148148147e-3) * t27482;
    let t27484 = t110 * t2238;
    let t27486 = F::new(0.15445601851851851852e-3) * t2237 * t27484;
    let t27487 = F::new(0.46336805555555555556e-3) * t7908 * t27455 - F::new(0.46336805555555555556e-3) * t27459 * t7911 + F::new(0.33163888888888888888e-2) * t27462 + F::new(0.24872916666666666666e-2) * t27465 + F::new(0.69505208333333333333e-3) * t2237 * t27416 - F::new(0.13901041666666666667e-2) * t2237 * t27342 + F::new(0.61836467013888888889e-4) * t27471 - F::new(0.2782641015625e-3) * t7898 * t27342 - F::new(0.49745833333333333332e-2) * t27477 + F::new(0.33163888888888888888e-2) * t27480 - t27483 + t27486;
    (t27480, t27482, t27483, t27484, t27486, t27487)
}
