//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1087/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1087<F: Float>(t157: F, t1838: F, t406: F, t1165: F, t2068: F, t604: F, t1815: F, t301: F, t1181: F, t30698: F, t599: F, t1479: F, t535: F) -> (F, F, F, F, F) {
    let t38784 = t1838 * t406 * t157;
    let t38787 = t2068 * t1165 * t604 * t38784;
    let t38789 = t1815 * t301;
    let t38792 = t30698 * t1181 * t599 * t38789;
    let t38795 = t535 * t1479;
    (t38784, t38787, t38789, t38792, t38795)
}
