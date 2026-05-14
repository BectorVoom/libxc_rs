//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 243/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk243<F: Float>(t140: F, t1013: F, t550: F, t133: F, t1010: F) -> (F, F) {
    let t141 = 0.1e-59 < t140;
    let t1014 = t550 * t1013;
    let t1015 = t133 * t1014;
    let t1017 = piecewise3(t141, 2.0 * t1010 - t1015, 0.0);
    (t1014, t1017)
}
