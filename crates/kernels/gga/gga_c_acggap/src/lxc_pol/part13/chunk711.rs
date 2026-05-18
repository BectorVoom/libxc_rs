//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 711/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk711<F: Float>(t1084: F, t1181: F, t7351: F, t7564: F, t1111: F, t604: F, t7426: F, t2070: F, t7433: F, t2450: F, t7336: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7566 = t1181 * t7351 * t1084;
    let t7567 = t7564 * t7566;
    let t7569 = t604 * t1111;
    let t7570 = t1181 * t7569;
    let t7571 = t7426 * t7570;
    let t7572 = F::new(0.42874018118069736972e-3) * t7571;
    let t7573 = t7433 * t2070;
    let t7574 = F::new(0.12862205435420921092e-2) * t7573;
    let t7575 = t2450 * t7336;
    (t7566, t7567, t7569, t7570, t7571, t7572, t7573, t7574, t7575)
}
