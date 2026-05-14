//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 920/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk920<F: Float>(t36177: F, t1181: F, t22275: F, t604: F, t7493: F, t1165: F, t23745: F, t7351: F, t2035: F, t31010: F, t35246: F, t30780: F, t35225: F, t1439: F, t1992: F, t1460: F, t30148: F) -> (F, F, F, F, F, F, F, F) {
    let t36178 = 0.34299214494455789578e-2 * t36177;
    let t36194 = t7493 * t1181 * t604 * t22275;
    let t36195 = 0.31448092289604152068e-2 * t36194;
    let t36198 = t7493 * t1165 * t7351 * t23745;
    let t36199 = 0.47172138434406228102e-2 * t36198;
    let t36205 = t2035 * t31010 * t35246;
    let t36206 = 0.183375e0 * t36205;
    let t36207 = t30780 * t35225;
    let t36208 = 0.916875e-1 * t36207;
    let t36209 = t1992 * t1439;
    let t36210 = t30780 * t36209;
    let t36211 = 0.916875e-1 * t36210;
    let t36213 = t30148 * t1460;
    (t36178, t36195, t36199, t36206, t36208, t36209, t36211, t36213)
}
