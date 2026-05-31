//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 977/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk977<F: Float>(t104: F, t9081: F, t694: F, t9090: F, t467: F, t9089: F, t1268: F, t560: F, t9083: F, t96: F, t10409: F, t10956: F, t14974: F, t19409: F, t1954: F, t2355: F, t24623: F, t2541: F, t3984: F, t567: F, t7292: F, t7297: F, t8027: F, t8382: F, t9096: F, t9097: F) -> F {
    let t33352 = t104 * t9081;
    let t33357 = F::cast_from(6.0_f64) * t694 * t9090;
    let t33358 = t9089 * t467;
    let t33383 = t560 * t1268;
    let t33388 = F::cast_from(2.0_f64) * t96 * t9083;
    let t33389 = -F::cast_from(6.0_f64) * t10409 * t7297 * t9089 - F::cast_from(6.0_f64) * t10956 * t3984 * t7297 - F::cast_from(3.0_f64) * t14974 * t2541 * t7297 - F::cast_from(6.0_f64) * t19409 * t2541 * t7297 + F::cast_from(6.0_f64) * t1954 * t33352 * t567 + F::cast_from(2.0_f64) * t2355 * t567 * t8027 + F::cast_from(6.0_f64) * t24623 * t7297 * t9097 + F::cast_from(12.0_f64) * t33358 * t7297 * t9097 + F::cast_from(2.0_f64) * t33383 * t9096 * t9097 + F::cast_from(6.0_f64) * t567 * t7292 * t8382 - t33357 + t33388;
    t33389
}
