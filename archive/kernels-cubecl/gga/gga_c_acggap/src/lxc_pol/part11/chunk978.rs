//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 978/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk978<F: Float>(t4099: F, t469: F, t1427: F, t467: F, t1662: F, t1679: F, t2541: F, t29948: F, t495: F, t694: F, t1298: F, t7278: F) -> (F, F, F, F, F) {
    let t33393 = t469 * t4099;
    let t33397 = t1427 * t467;
    let t33403 = F::cast_from(2.0_f64) * t1679 * t2541 * t1662;
    let t33409 = F::cast_from(6.0_f64) * t694 * t29948 * t495;
    let t33412 = F::cast_from(6.0_f64) * t694 * t7278 * t1298;
    (t33393, t33397, t33403, t33409, t33412)
}
