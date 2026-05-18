//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 544/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk544<F: Float>(t1165: F, t3253: F, t3457: F, t3456: F, t1172: F, t1530: F, t396: F, t980: F, t409: F, t932: F, t935: F, t322: F, t922: F) -> (F, F, F, F, F, F, F) {
    let t3459 = t1165 * t3253 * t3457;
    let t3460 = t3456 * t3459;
    let t3462 = t1530 * t1172;
    let t3476 = t980 * t396;
    let t3477 = t3476 * t409;
    let t3479 = t935 * t932;
    let t3491 = t922 * t322;
    (t3459, t3460, t3462, t3476, t3477, t3479, t3491)
}
