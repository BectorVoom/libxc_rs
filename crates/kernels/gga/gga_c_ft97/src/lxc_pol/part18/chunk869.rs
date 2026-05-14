//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 869/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk869<F: Float>(t23405: F, t5775: F, t165: F, t5842: F, t379: F, t1969: F, t5765: F, t92: F) -> (F, F, F, F) {
    let t23406 = t23405 * t5775;
    let t23408 = t5842 * t165;
    let t23409 = t23408 * t379;
    let t23410 = t1969 * t23409;
    let t23413 = t5765 * t92;
    (t23406, t23408, t23410, t23413)
}
