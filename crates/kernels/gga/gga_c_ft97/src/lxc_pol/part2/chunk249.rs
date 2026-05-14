//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 249/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk249<F: Float>(t871: F, t875: F, t296: F, t193: F, t446: F, t834: F, t837: F, t842: F, t865: F, t89: F, t312: F, t863: F) -> (F, F, F, F) {
    let t876 = t871 * t875;
    let t877 = t296 * t876;
    let t880 = -t834 - t446 * t837 / 9.0 - t446 * t842 / 3.0 + t89 * t193 * t865 / 3.0 - t446 * t877 / 3.0;
    let t882 = t863 * t312;
    (t876, t877, t880, t882)
}
