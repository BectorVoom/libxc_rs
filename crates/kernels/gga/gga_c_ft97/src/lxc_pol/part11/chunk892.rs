//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 892/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk892<F: Float>(t1559: F, t1986: F, t8392: F, t9350: F, t9355: F, t144: F, t167: F, t1901: F, t2185: F, t2221: F, t2230: F, t3434: F, t3440: F, t39641: F, t40262: F, t40696: F, t40698: F, t40700: F, t40720: F, t446: F, t574: F, t616: F, t9007: F, t9133: F) -> (F, F) {
    let t40722 = t1559 * t1986;
    let t40727 = t8392 * t9350;
    let t40729 = t8392 * t9355;
    let t40731 = 112.0 / 81.0 * t40696 + 112.0 / 81.0 * t40698 + 8.0 / 3.0 * t1901 * t2221 * t3440 * t40700 - 2.0 * t446 * t144 * t39641 - 4.0 / 3.0 * t446 * t574 * t616 * t9007 - t446 * t574 * t167 * t40262 / 3.0 + 4.0 * t446 * t2185 * t2230 * t1986 + 8.0 / 9.0 * t40720 + 8.0 / 3.0 * t1901 * t9133 * t3434 * t40722 - 4.0 / 9.0 * t40727 - 4.0 / 9.0 * t40729;
    (t40722, t40731)
}
