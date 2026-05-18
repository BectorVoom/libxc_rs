//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1109/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1109<F: Float>(t1423: F, t7736: F, t30318: F, t542: F, t2001: F, t4886: F, t2327: F, t7630: F, t13287: F, t31057: F, t35700: F, t2288: F, t3196: F) -> (F, F, F, F, F, F) {
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35742 = t2001 * t4886;
    let t35744 = t7630 * t2327;
    let t35747 = t31057 * t13287 * t35700;
    let t35749 = t2288 * t3196;
    (t35738, t35740, t35742, t35744, t35747, t35749)
}
