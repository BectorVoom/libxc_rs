//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 514/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk514<F: Float>(t1064: F, t2779: F, t2778: F, t550: F, t1365: F, t599: F, t986: F) -> (F, F, F, F) {
    let t2780 = t1064 * t2779;
    let t2783 = t550 * t2778;
    let t2784 = t1365 * t2783;
    let t2787 = t599 * t986;
    (t2780, t2783, t2784, t2787)
}
