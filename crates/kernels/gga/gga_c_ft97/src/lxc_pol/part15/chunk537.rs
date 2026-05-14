//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 537/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk537<F: Float>(t8189: F, t2: F, t7241: F, t7760: F, t82: F, t7763: F, t1542: F, t17: F, t9: F) -> (F, F, F, F, F) {
    let t8260 = 28.0 / 27.0 * t8189;
    let t8270 = t7241 * t2;
    let t8275 = t7760 * t82;
    let t8276 = t2 * t7763;
    let t8281 = t1542 * t17;
    let t8282 = t9 * t8281;
    (t8260, t8270, t8275, t8276, t8282)
}
