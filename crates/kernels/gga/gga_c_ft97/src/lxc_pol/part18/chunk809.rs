//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 809/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk809<F: Float>(t408: F, t5532: F, t428: F, t1751: F, t5522: F, t1691: F, sigma0: F) -> (F, F, F, F) {
    let t22701 = t408 * t5532;
    let t22702 = t22701 * t428;
    let t22705 = t5522 * t1751;
    let t22708 = t1691 * sigma0;
    (t22701, t22702, t22705, t22708)
}
