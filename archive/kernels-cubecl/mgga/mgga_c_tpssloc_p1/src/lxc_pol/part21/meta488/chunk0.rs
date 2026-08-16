//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2090/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2090<F: Float>(t16758: F, t829: F, t4234: F, t4282: F, t5550: F, t9573: F, t213: F, t5527: F, t221: F, t776: F, t4119: F, t4128: F) -> (F, F, F, F, F, F) {
    let t16759 = t16758 * t829;
    let t16762 = t4282 * t4234;
    let t16769 = t9573 * t5550;
    let t16771 = t213 * t5527;
    let t16773 = t221 * t16771 * t776;
    let t16777 = t221 * t4128 * t4119;
    (t16759, t16762, t16769, t16771, t16773, t16777)
}
