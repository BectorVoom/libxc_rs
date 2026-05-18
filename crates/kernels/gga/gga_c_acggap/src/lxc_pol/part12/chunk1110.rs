//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1110/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1110<F: Float>(t13287: F, t31195: F, t35749: F, t2001: F, t4724: F, t1429: F, t7605: F, t1165: F, t20590: F, t604: F, t7337: F, t5272: F, t7561: F) -> (F, F, F, F, F) {
    let t35751 = t31195 * t13287 * t35749;
    let t35753 = t2001 * t4724;
    let t35755 = t7605 * t1429;
    let t35759 = t7337 * t1165 * t604 * t20590;
    let t35766 = t7561 * t5272;
    (t35751, t35753, t35755, t35759, t35766)
}
