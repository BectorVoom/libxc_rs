//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 975/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk975<F: Float>(t670: F, t8342: F, t117: F, t8320: F, t1459: F, t1461: F, t2207: F, t2209: F, t572: F, t573: F, t8336: F, t1843: F, t2198: F) -> (F, F, F, F) {
    let t8343 = t8342 * t670;
    let t8346 = t117 * t8320;
    let t8349 = F::cast_from(3.0_f64) * t1459 * t2209 + F::cast_from(3.0_f64) * t1461 * t2207 + F::cast_from(6.0_f64) * t572 * t8343 + F::cast_from(3.0_f64) * t572 * t8346 + t573 * t8336;
    let t8393 = t1843 * t2198;
    (t8343, t8346, t8349, t8393)
}
