//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 502/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk502<F: Float>(t6861: F, t729: F, t762: F, t265: F, t6837: F, t1091: F, t1456: F, t724: F, t1131: F, t1154: F, t1424: F) -> (F, F, F, F, F) {
    let t6863 = t729 * t762 * t6861;
    let t6867 = t729 * t265 * t6837;
    let t6871 = t724 * t1456 * t1091;
    let t6875 = t729 * t1456 * t1131;
    let t6878 = t1424 * t1154;
    (t6863, t6867, t6871, t6875, t6878)
}
