//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 869/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk869<F: Float>(t231: F, t27617: F, t213: F, t679: F, t689: F, t6979: F, t709: F, t3817: F, t6027: F, t2441: F, t3886: F, t6035: F, t1127: F, t703: F, t684: F, t1119: F, t70: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27618 = t27617 * t231;
    let t27619 = t213 * t679;
    let t27620 = t27619 * t689;
    let t27621 = t27618 * t27620;
    let t27625 = t6979 * t709;
    let t27629 = t6027 * t3817;
    let t27633 = t2441 * t3886;
    let t27634 = t6035 * t27633;
    let t27637 = t703 * t1127;
    let t27638 = t27637 * t684;
    let t27642 = t1119 * t70;
    (t27618, t27620, t27621, t27625, t27629, t27633, t27634, t27637, t27638, t27642)
}
