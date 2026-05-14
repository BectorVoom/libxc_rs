//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 341/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk341<F: Float>(t1033: F, t221: F, t643: F, t954: F, t642: F, t639: F, t1027: F, t657: F, t1029: F, t25: F, t651: F, t655: F) -> (F, F, F, F, F, F) {
    let t1035 = 2.0 / 15.0 * t1033 * t221;
    let t1036 = t643 * t954;
    let t1037 = t642 * t1036;
    let t1039 = 4.0 / 45.0 * t639 * t1037;
    let t1041 = t657 * t1027;
    let t1044 = -t651 - 0.35991666666666666667e-1 * t1029 - t655 - 0.66666666666666666667e-2 * t25 * t1041;
    (t1035, t1036, t1037, t1039, t1041, t1044)
}
