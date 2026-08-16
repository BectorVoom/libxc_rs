//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 406/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk406<F: Float>(t1073: F, t2281: F, t184: F, t21: F, t1087: F, t2336: F, t89: F, t2347: F, t992: F) -> (F, F, F, F) {
    let t3640 = t2281 * t1073;
    let t3664 = t184 * t21;
    let t3688 = t89 * t2336 * t1087;
    let t3690 = t2347 * t992;
    (t3640, t3664, t3688, t3690)
}
