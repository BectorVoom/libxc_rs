//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1222/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1222<F: Float>(t16784: F, t1765: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t12349: F, t12352: F, t16708: F, t16893: F, t16929: F, t16931: F, t20366: F, t20368: F, t20371: F, t20373: F, t20378: F) -> (F, F, F) {
    let t20404 = 0.11696447245269292414e1 * t16784 * t1765;
    let t20425 = 0.66437037037037037037e-1 * t20283 - 0.19931111111111111111e0 * t20285 - 0.99655555555555555557e-1 * t20287 + 0.29896666666666666667e0 * t20290 + 0.33218518518518518518e0 * t20295 - 0.11958666666666666667e1 * t20300 - 0.39862222222222222222e0 * t20304 + 0.17938e1 * t20308 + 0.11958666666666666667e1 * t20312 - 0.19931111111111111111e0 * t20315 + 0.59793333333333333334e0 * t20320;
    let t20445 = -0.76790625e-1 * t20366 + 0.3071625e0 * t20368 + 0.15358125e0 * t20371 - t16893 - t12349 - t12352 + 0.3071625e0 * t20373 - t16929 + 0.13287407407407407407e0 * t16708 + 0.36514074074074074073e-1 * t16931 + 0.36514074074074074075e-1 * t20378;
    (t20404, t20425, t20445)
}
