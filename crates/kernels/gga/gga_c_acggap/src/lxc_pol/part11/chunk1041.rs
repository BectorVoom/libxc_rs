//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1041/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1041<F: Float>(t1181: F, t22107: F, t604: F, t8463: F, t1165: F, t4257: F, t7351: F, t22275: F, t7493: F, t23745: F, t2068: F, t21128: F, t2035: F, t31010: F, t35246: F, t30780: F, t35225: F) -> (F, F, F, F, F, F, F) {
    let t36186 = t8463 * t1181 * t604 * t22107;
    let t36190 = t8463 * t1165 * t7351 * t4257;
    let t36194 = t7493 * t1181 * t604 * t22275;
    let t36195 = 0.31448092289604152068e-2 * t36194;
    let t36198 = t7493 * t1165 * t7351 * t23745;
    let t36199 = 0.47172138434406228102e-2 * t36198;
    let t36202 = t2068 * t1165 * t7351 * t21128;
    let t36205 = t2035 * t31010 * t35246;
    let t36206 = 0.183375e0 * t36205;
    let t36207 = t30780 * t35225;
    (t36186, t36190, t36195, t36199, t36202, t36206, t36207)
}
