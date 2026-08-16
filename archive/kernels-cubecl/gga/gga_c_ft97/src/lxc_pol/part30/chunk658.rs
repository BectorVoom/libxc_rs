//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 658/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk658<F: Float>(t1208: F, t703: F, t684: F, t811: F, t992: F, t704: F, t27575: F, t7009: F, t24330: F, t6999: F, t6242: F, t7006: F) -> (F, F, F, F, F, F) {
    let t28561 = t703 * t1208;
    let t28562 = t28561 * t684;
    let t28566 = t992 * t811;
    let t28567 = t704 * t28566;
    let t28572 = t7009 * t27575;
    let t28574 = t24330 * t6999;
    let t28575 = t6242 * t28574;
    let t28577 = t7006 * t27575;
    (t28562, t28566, t28567, t28572, t28575, t28577)
}
