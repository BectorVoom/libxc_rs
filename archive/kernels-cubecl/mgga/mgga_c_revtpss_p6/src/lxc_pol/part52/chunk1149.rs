//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1149/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1149<F: Float>(t10073: F, t25403: F, t32433: F, t120066: F, t120070: F, t120073: F, t121920: F, t32474: F, t119971: F, t121834: F, t251: F, t31837: F) -> (F, F, F, F, F, F) {
    let t121980 = F::cast_from(0.4818682326780666368e-3_f64) * t10073 * t32433 * t25403;
    let t121990 = F::cast_from(0.14932895752263002547e-1_f64) * t120066;
    let t121991 = F::cast_from(0.40155686056505553065e-3_f64) * t120070;
    let t121992 = F::cast_from(0.71396809808466873356e-3_f64) * t120073;
    let t121993 = t32474 * t121920;
    let t121998 = F::cast_from(0.6019057092162847523e-2_f64) * t119971 * t251 * t31837 * t121834;
    (t121980, t121990, t121991, t121992, t121993, t121998)
}
