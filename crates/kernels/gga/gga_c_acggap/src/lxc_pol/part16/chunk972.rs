//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 972/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk972<F: Float>(t34421: F, t1988: F, t8536: F, t2278: F, t7600: F, t2290: F, t7610: F, t30374: F, t8477: F, t1181: F, t4555: F, t599: F, t7493: F) -> (F, F, F, F, F, F) {
    let t34422 = F::new(7.0) / F::new(144.0) * t34421;
    let t34429 = t1988 * t8536;
    let t34430 = F::new(0.10718504529517434243e-2) * t34429;
    let t34433 = t7600 * t2278;
    let t34435 = t7610 * t2290;
    let t34449 = t30374 * t8477;
    let t34453 = t7493 * t1181 * t599 * t4555;
    (t34422, t34430, t34433, t34435, t34449, t34453)
}
