//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 579/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk579<F: Float>(t1096: F, t684: F, t24278: F, t420: F, t704: F, t679: F, t992: F, t689: F, t17864: F, t6023: F, t3766: F, t6042: F) -> (F, F, F, F, F, F) {
    let t27533 = t1096 * t684;
    let t27534 = t24278 * t27533;
    let t27537 = t420 * t704;
    let t27538 = t992 * t679;
    let t27539 = t27538 * t689;
    let t27540 = t27537 * t27539;
    let t27543 = t6023 * t17864;
    let t27546 = t3766 * t6042;
    (t27533, t27534, t27539, t27540, t27543, t27546)
}
