//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 644/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk644<F: Float>(t1865: F, t321: F, t1: F, t787: F, t723: F, t835: F, t121: F, t2066: F) -> (F, F, F, F) {
    let t6115 = t321 * t1865;
    let t6116 = t6115 * t1;
    let t6117 = t787 * t6116;
    let t6125 = t835 * t723;
    let t6134 = t2066 * t121;
    (t6115, t6117, t6125, t6134)
}
