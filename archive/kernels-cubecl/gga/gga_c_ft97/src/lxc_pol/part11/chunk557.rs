//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 557/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk557<F: Float>(t12: F, t52: F, t1703: F, t1593: F, t1609: F, t1595: F, t1597: F, t63: F, t1620: F, t5544: F, t39: F, t409: F) -> (F, F, F, F, F, F, F, F) {
    let t7853 = t52 * t12;
    let t7854 = t7853 * t1703;
    let t7857 = t1609 * t1593;
    let t7858 = t1595 * t1597;
    let t7859 = t7858 * t63;
    let t7860 = t7857 * t7859;
    let t7861 = t5544 * t1620;
    let t7866 = t409 * t39;
    (t7853, t7854, t7857, t7858, t7859, t7860, t7861, t7866)
}
