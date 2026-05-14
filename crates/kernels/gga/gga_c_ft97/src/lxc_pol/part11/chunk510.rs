//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 510/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk510<F: Float>(t2347: F, t258: F, t2: F, t2486: F, t737: F, t284: F, t811: F, t285: F, t287: F, t2766: F, t309: F) -> (F, F, F, F, F, F) {
    let t3892 = t258 * t2347;
    let t3910 = t2486 * t2;
    let t3917 = t737 * t2;
    let t4061 = t811 * t284;
    let t4113 = t285 * t287;
    let t4139 = t2766 * t309;
    (t3892, t3910, t3917, t4061, t4113, t4139)
}
