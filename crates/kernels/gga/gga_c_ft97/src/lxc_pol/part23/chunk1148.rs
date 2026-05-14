//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1148/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1148<F: Float>(t28309: F, t8392: F, t53891: F, t6074: F, t24737: F, t53798: F, t6923: F, t8232: F, t6858: F, t1882: F, t28260: F, t28438: F, t28137: F, t6867: F, t28369: F, t28375: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t111070 = 2.0 / 27.0 * t8392 * t28309;
    let t111085 = t53891 * t6074;
    let t111089 = t53798 * t24737;
    let t111109 = t8232 * t6923;
    let t111111 = t8232 * t6858;
    let t111121 = 4.0 / 9.0 * t1882 * t28260;
    let t111137 = 4.0 / 9.0 * t1882 * t28438;
    let t111190 = 4.0 / 9.0 * t8392 * t28137;
    let t111215 = t8232 * t6867;
    let t111221 = 4.0 / 9.0 * t8392 * t28369;
    let t111223 = 4.0 / 27.0 * t8392 * t28375;
    (t111070, t111085, t111089, t111109, t111111, t111121, t111137, t111190, t111215, t111221, t111223)
}
