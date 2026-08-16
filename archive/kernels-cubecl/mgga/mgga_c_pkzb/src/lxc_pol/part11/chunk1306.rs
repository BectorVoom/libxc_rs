//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1306/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1306<F: Float>(t31390: F, t31394: F, t31397: F, t31400: F, t31404: F, t31407: F, t31411: F, t31643: F, t31647: F, t31650: F, t31653: F, t10106: F, t300: F) -> (F, F) {
    let t31654 = t31390 - t31394 - t31397 - t31400 + t31404 + t31407 + t31411 - t31643 - t31647 - t31650 - t31653;
    let t31668 = t300 * t10106;
    (t31654, t31668)
}
