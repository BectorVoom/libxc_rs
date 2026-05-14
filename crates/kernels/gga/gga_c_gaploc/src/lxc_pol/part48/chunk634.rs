//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 634/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk634<F: Float>(t13525: F, t169: F, t299: F, t706: F, t1035: F, t3433: F, t13177: F, t13488: F, t13490: F, t13494: F, t13497: F, t13498: F, t13501: F, t13504: F, t13509: F, t2508: F, t270: F) -> (F, F, F, F) {
    let t13527 = t13525 * t169 * t299;
    let t13528 = t706 * t13527;
    let t13531 = t1035 * t3433;
    let t13534 = 0.1281754371690370714e-2 * t13177 - t13488 - 0.96131577876777803546e-3 * t13490 + t13494 + t13497 - 0.46143157380853345702e-1 * t2508 * t13498 + t13501 + 0.64087718584518535696e-3 * t13504 - t13509 + 0.76905262301422242837e-2 * t270 * t13528 + 0.15381052460284448567e-1 * t2508 * t13531;
    (t13527, t13528, t13531, t13534)
}
