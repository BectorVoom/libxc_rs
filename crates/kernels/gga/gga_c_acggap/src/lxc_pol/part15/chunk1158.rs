//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1158/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1158<F: Float>(t7447: F, t9701: F, t7440: F, t9705: F, t1314: F, t507: F, t8806: F, t34406: F, t6324: F, t8463: F, t8480: F, t8652: F) -> (F, F, F, F, F) {
    let t40045 = t7447 * t9701;
    let t40047 = t7440 * t9705;
    let t40050 = t8806 * t507 * t1314;
    let t40054 = t34406 * t6324;
    let t40057 = t8463 * t8480 * t8652;
    (t40045, t40047, t40050, t40054, t40057)
}
