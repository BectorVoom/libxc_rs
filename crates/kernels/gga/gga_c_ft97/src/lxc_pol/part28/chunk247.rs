//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 247/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk247<F: Float>(t118: F, t29: F, t341: F, t343: F, t123: F, t532: F, t129: F, t39: F, t11: F, t1689: F) -> (F, F, F, F, F) {
    let t2007 = 1.0 / t118 / t29;
    let t2014 = t341 * t343;
    let t2021 = t123 / t532 / t29;
    let t2034 = t129 * t39;
    let t2035 = t1689 * t11;
    (t2007, t2014, t2021, t2034, t2035)
}
