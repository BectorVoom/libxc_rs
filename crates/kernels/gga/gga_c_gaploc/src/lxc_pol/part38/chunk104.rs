//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 104/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk104<F: Float>(t492: F, t494: F, t105: F, t174: F, t337: F, t359: F, t364: F, t377: F, t380: F, t419: F, t449: F, t478: F, t484: F, t489: F, t208: F) -> (F, F, F) {
    let t495 = t492 * t494;
    let t498 = t337 + t359 - t364 - t377 + 0.37940008847568199465e-1 * t380 * t174 + 0.28455006635676149599e-1 * t419 * t174 - 0.28455006635676149599e-1 * t105 * t449 + 0.28455006635676149599e-1 * t105 * t478 - 0.31616674039640166221e-2 * t484 * t489 - 0.28455006635676149599e-1 * t105 * t495;
    let t500 = t208 * t208;
    let t501 = 1.0 / t500;
    (t498, t500, t501)
}
