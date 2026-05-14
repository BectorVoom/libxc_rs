//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 738/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk738<F: Float>(t299: F, t10943: F, t7858: F, t7906: F, t383: F, t7857: F, t1598: F, t66: F, t1593: F, t1595: F, t1630: F, t14: F, t7741: F, t12: F) -> (F, F, F, F, F, F, F, F) {
    let t300 = 10000000.0 <= t299;
    let t10944 = piecewise3(t300, 0.0, t10943);
    let t11109 = t7906 * t7858;
    let t11119 = t7857 * t383;
    let t11120 = t1598 * t66;
    let t11121 = t11119 * t11120;
    let t11140 = t1593 * t1595;
    let t11153 = t1630 * t1595;
    let t11174 = 1.0 / t14 / t7741;
    let t11175 = t12 * t11174;
    (t10944, t11109, t11120, t11121, t11140, t11153, t11174, t11175)
}
