//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2129/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2129<F: Float>(t27382: F, t98633: F, t198: F, t206: F, t7782: F, t2: F, t892: F, t580: F, t775: F, t25206: F, t1583: F, t2430: F) -> (F, F, F, F) {
    let t98635 = F::new(2.0) * t27382 * t98633;
    let t98637 = t198 * t206 * t7782;
    let t98646 = t892 * t2;
    let t98648 = t98646 * t580 * t775;
    let t98650 = F::new(6.0) * t25206 * t98648;
    let t98651 = t1583 * t2430;
    (t98635, t98637, t98650, t98651)
}
