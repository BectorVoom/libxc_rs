//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 366/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk366<F: Float>(t2179: F, t342: F, t303: F, t2173: F, t2175: F, t393: F, t374: F, t377: F, t1021: F, t389: F) -> (F, F, F, F, F, F, F) {
    let t2180 = t342 * t2179;
    let t2181 = t303 * t2180;
    let t2183 = -F::cast_from(0.69505208333333333333e-3_f64) * t2173 * t2175 + F::cast_from(0.24872916666666666666e-2_f64) * t2181;
    let t2184 = t2183 * t393;
    let t2185 = t374 * t377;
    let t2187 = t1021 * t389;
    let t2189 = t2185 / F::cast_from(16.0_f64) - t2187 / F::cast_from(128.0_f64);
    (t2180, t2181, t2183, t2184, t2185, t2187, t2189)
}
