//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1147/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1147<F: Float>(t13287: F, t31057: F, t35700: F, t2288: F, t3196: F, t31195: F, t2001: F, t4724: F, t1429: F, t7605: F, t1165: F, t20590: F, t604: F, t7337: F) -> (F, F, F, F, F, F) {
    let t35747 = t31057 * t13287 * t35700;
    let t35748 = F::new(0.42874018118069736972e-3) * t35747;
    let t35749 = t2288 * t3196;
    let t35751 = t31195 * t13287 * t35749;
    let t35753 = t2001 * t4724;
    let t35755 = t7605 * t1429;
    let t35756 = F::new(0.17149607247227894789e-1) * t35755;
    let t35759 = t7337 * t1165 * t604 * t20590;
    (t35748, t35749, t35751, t35753, t35756, t35759)
}
