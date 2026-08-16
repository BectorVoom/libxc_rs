//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1879/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1879<F: Float>(t1307: F, t6637: F, t6888: F, t97126: F, t26331: F, t26446: F, t96964: F, t28164: F, t6914: F, t22704: F, t22705: F, t28181: F) -> (F, F, F, F) {
    let t97129 = t6888 * t6637 * t97126 * t1307;
    let t97135 = t26331 * t26446 * t96964 * t1307;
    let t97137 = t6914 * t28164;
    let t97142 = t22704 * t22705 * t28181;
    (t97129, t97135, t97137, t97142)
}
