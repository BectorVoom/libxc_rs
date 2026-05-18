//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1017/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1017<F: Float>(t2327: F, t7630: F, t13287: F, t31057: F, t35700: F, t1429: F, t7605: F, t31593: F, t30219: F, t8469: F, t1562: F, t31824: F) -> (F, F, F, F, F, F) {
    let t35744 = t7630 * t2327;
    let t35747 = t31057 * t13287 * t35700;
    let t35755 = t7605 * t1429;
    let t35764 = F::new(0.42874018118069736972e-3) * t31593;
    let t35774 = t30219 * t8469;
    let t35784 = t31824 * t1562;
    (t35744, t35747, t35755, t35764, t35774, t35784)
}
