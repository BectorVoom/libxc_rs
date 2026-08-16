//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 817/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk817<F: Float>(t15763: F, t2205: F, t446: F, t1017: F, t3052: F, t1969: F, t3281: F, t4458: F, t558: F, t15752: F, t569: F, t4454: F) -> (F, F, F, F, F, F, F) {
    let t16705 = t2205 * t15763;
    let t16706 = t446 * t16705;
    let t16708 = t3052 * t1017;
    let t16709 = t1969 * t16708;
    let t16710 = t3281 * t16709;
    let t16712 = t4458 * t558;
    let t16713 = t1969 * t16712;
    let t16714 = t446 * t16713;
    let t16716 = t569 * t15752;
    let t16717 = t446 * t16716;
    let t16719 = t4454 * t558;
    (t16706, t16708, t16710, t16712, t16714, t16717, t16719)
}
