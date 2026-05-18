//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 87/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk87<F: Float>(t70: F, t71: F, t64: F, t1: F, t341: F, t67: F, t345: F, t347: F, t14: F, t344: F, t337: F, t359: F, t364: F, t377: F) -> (F, F, F) {
    let t386 = F::new(1.0) / t71 / t70;
    let t387 = t64 * t386;
    let t389 = t341 * t67 * t1;
    let t394 = -F::new(0.66066666666666666667e-2) * t345 - F::new(0.41275e-2) * t347;
    let t397 = -t389 * t344 / F::new(12.0) + t14 * t394 / F::new(2.0);
    let t400 = t337 + t359 - t364 - t377;
    (t387, t397, t400)
}
