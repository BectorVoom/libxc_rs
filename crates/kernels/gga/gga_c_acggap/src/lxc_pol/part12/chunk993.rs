//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 993/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk993<F: Float>(t467: F, t9089: F, t1268: F, t560: F, t4099: F, t469: F, t1427: F, t1603: F, t618: F, t2137: F, t525: F, t879: F) -> (F, F, F, F, F, F, F) {
    let t33358 = t9089 * t467;
    let t33383 = t560 * t1268;
    let t33393 = t469 * t4099;
    let t33397 = t1427 * t467;
    let t33428 = t1603 * t618;
    let t33429 = t2137 * t33428;
    let t33509 = t525 * t879;
    (t33358, t33383, t33393, t33397, t33428, t33429, t33509)
}
