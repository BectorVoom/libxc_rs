//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1911/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1911<F: Float>(t98229: F, t98235: F, t98238: F, t98243: F, t98258: F, t98269: F, t98281: F, t1904: F, t2439: F, t26358: F, t213: F, t28888: F) -> (F, F, F, F, F, F, F, F, F) {
    let t102531 = F::cast_from(0.22866142996303859718e-3_f64) * t98229;
    let t102534 = F::cast_from(0.22866142996303859718e-3_f64) * t98235;
    let t102535 = F::cast_from(0.57165357490759649296e-4_f64) * t98238;
    let t102537 = F::cast_from(0.2032800112371413129e-3_f64) * t98243;
    let t102548 = F::cast_from(0.11433071498151929859e-3_f64) * t98258;
    let t102557 = F::new(7.0) / F::new(36.0) * t98269;
    let t102567 = F::cast_from(0.22866142996303859718e-3_f64) * t98281;
    let t102582 = t2439 * t26358 * t1904;
    let t102594 = t213 * t28888;
    (t102531, t102534, t102535, t102537, t102548, t102557, t102567, t102582, t102594)
}
