//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 605/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk605<F: Float>(t4829: F, t8392: F, t1045: F, t2097: F, t4805: F, t604: F, t16679: F, t4753: F, t9252: F, t16745: F, t16748: F, t16751: F, t2: F, t4668: F, t1775: F, t4765: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17104 = t8392 * t4829;
    let t17164 = t2097 * t1045;
    let t17198 = t604 * t4805;
    let t17214 = 2.0 / 9.0 * t16679;
    let t17239 = t9252 * t4753;
    let t17249 = t16745 / 9.0;
    let t17250 = 2.0 / 9.0 * t16748;
    let t17251 = 2.0 / 27.0 * t16751;
    let t17259 = t2 * t4668;
    let t17272 = t1775 * t4765;
    (t17104, t17164, t17198, t17214, t17239, t17249, t17250, t17251, t17259, t17272)
}
