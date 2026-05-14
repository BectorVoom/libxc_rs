//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1152/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1152<F: Float>(t100294: F, t22952: F, t22953: F, t26006: F, t379: F, t11618: F, t38921: F, t5674: F, t5675: F, t11176: F, t1316: F, t25880: F, t23054: F, t25875: F, t1882: F, t25972: F) -> (F, F, F, F, F, F, F) {
    let t100295 = 2.0 / 27.0 * t100294;
    let t100298 = t22952 * t22953 * t26006 * t379;
    let t100302 = t5674 * t38921 * t5675 * t11618;
    let t100305 = t1316 * t11176 * t25880;
    let t100307 = t23054 * t25875;
    let t100308 = t100307 / 27.0;
    let t100309 = t1882 * t25972;
    (t100295, t100298, t100302, t100305, t100307, t100308, t100309)
}
