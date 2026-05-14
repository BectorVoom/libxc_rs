//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 345/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk345<F: Float>(t1027: F, t625: F, t11: F, t624: F, t203: F, t184: F) -> (F, F, F, F, F) {
    let t1028 = t625 * t1027;
    let t1029 = t11 * t1028;
    let t1031 = t624 + 0.18891666666666666667e-2 * t1029;
    let t1032 = t203 * t1031;
    let t1033 = t1032 * t184;
    (t1028, t1029, t1031, t1032, t1033)
}
