//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 924/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk924<F: Float>(t7021: F, t870: F, t28842: F, t1495: F, t2681: F, t6353: F, t848: F, t108446: F, t3766: F, t27669: F, t79528: F, t226: F, t27703: F) -> (F, F, F, F, F, F, F) {
    let t114578 = t870 * t7021;
    let t114751 = t28842 * t870;
    let t114820 = t2681 * t1495;
    let t114847 = t848 * t6353;
    let t122830 = t3766 * t108446;
    let t123028 = t79528 * t27669;
    let t123124 = t27703 * t226;
    (t114578, t114751, t114820, t114847, t122830, t123028, t123124)
}
