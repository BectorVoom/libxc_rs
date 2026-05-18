//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 542/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk542<F: Float>(t408: F, t5532: F, t25: F, t1689: F, t39: F, t77: F, t1608: F) -> (F, F, F, F) {
    let t22701 = t408 * t5532;
    let t22718 = t5532 * t25;
    let t22735 = t77 * t39 * t1689;
    let t22736 = t1608 * t22735;
    (t22701, t22718, t22735, t22736)
}
