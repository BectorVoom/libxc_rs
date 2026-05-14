//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1014/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1014<F: Float>(t2001: F, t4552: F, t1998: F, t5089: F, t1451: F, t7605: F, t1423: F, t7736: F, t30318: F, t542: F, t4886: F, t2327: F, t7630: F, t13287: F, t31057: F, t35700: F) -> (F, F, F, F, F, F, F, F) {
    let t35731 = t2001 * t4552;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    let t35737 = 0.34299214494455789578e-2 * t35736;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    let t35742 = t2001 * t4886;
    let t35744 = t7630 * t2327;
    let t35747 = t31057 * t13287 * t35700;
    (t35731, t35733, t35737, t35738, t35740, t35742, t35744, t35747)
}
