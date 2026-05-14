//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1045/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1045<F: Float>(t5949: F, t8232: F, t1380: F, t7943: F, t89: F, t5970: F, t5937: F, t5886: F, t1637: F, t5931: F, t1378: F, t9114: F, t1386: F, t3281: F, t26513: F, t27462: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t96167 = t8232 * t5949;
    let t96215 = 28.0 / 81.0 * t89 * t7943 * t1380;
    let t96220 = t8232 * t5970;
    let t96222 = t8232 * t5937;
    let t96224 = t8232 * t5886;
    let t96227 = t89 * t1637 * t5931;
    let t96239 = t9114 * t1378;
    let t96244 = 28.0 / 81.0 * t3281 * t1386;
    let t100044 = 2.0 * t26513;
    let t100045 = 2.0 * t27462;
    (t96167, t96215, t96220, t96222, t96224, t96227, t96239, t96244, t100044, t100045)
}
