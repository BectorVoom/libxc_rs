//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 667/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk667<F: Float>(t3404: F, t72: F, t22632: F, t5829: F, t6608: F, t1013: F, t1701: F, t22652: F, t1008: F, t2035: F, t5790: F, t3347: F, t5784: F) -> (F, F, F, F, F, F) {
    let t26650 = t72 * t3404;
    let t26658 = t5829 * t22632 * t6608;
    let t26661 = t1701 * t22652 * t1013;
    let t26665 = t1701 * t22652 * t1008;
    let t26671 = t2035 * t5790 * t1013;
    let t26674 = t3347 * t5784;
    (t26650, t26658, t26661, t26665, t26671, t26674)
}
