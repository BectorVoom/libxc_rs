//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 746/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk746<F: Float>(t8945: F, t7772: F, t7774: F, t7790: F, t7798: F, t8268: F, t8269: F, t8271: F, t8275: F, t8276: F, t8278: F, t8943: F, t8949: F, t8953: F, t8957: F, t8973: F) -> (F, F) {
    let t9348 = 7.0 / 144.0 * t8945;
    let t9352 = -t7772 - t7774 - t8268 + t8269 - t8271 + t7790 + t7798 + t8275 - t8276 - t8278 + t8943 / 48.0 - t9348 + 0.42874018118069736972e-3 * t8949 - 0.31448092289604152069e-3 * t8953 - 0.15724046144802076034e-2 * t8957;
    let t9356 = 0.64311027177104605458e-2 * t8973;
    (t9352, t9356)
}
