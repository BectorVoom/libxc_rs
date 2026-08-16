//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 961/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk961<F: Float>(t7440: F, t7490: F, t4680: F, t7493: F, t7642: F, t1165: F, t14575: F, t604: F, t7346: F, t1089: F, t31520: F, t31521: F, t368: F) -> (F, F, F, F) {
    let t31790 = t7440 * t7490;
    let t31793 = t7493 * t4680 * t7642;
    let t31797 = t7346 * t1165 * t604 * t14575;
    let t31805 = t31520 * t1089 * t368 * t31521;
    (t31790, t31793, t31797, t31805)
}
