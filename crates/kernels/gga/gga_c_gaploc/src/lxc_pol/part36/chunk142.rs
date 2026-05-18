//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 142/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk142<F: Float>(t255: F, t256: F, t64: F, t1: F, t252: F, t341: F, t345: F, t347: F, t14: F, t344: F, t337: F, t359: F, t642: F, t645: F, t648: F) -> (F, F, F) {
    let t656 = F::new(1.0) / t256 / t255;
    let t657 = t64 * t656;
    let t659 = t341 * t252 * t1;
    let t664 = -F::new(0.14921166666666666667e-3) * t345 - F::new(0.39332083333333333333e-2) * t347;
    let t667 = -t659 * t344 / F::new(12.0) + t14 * t664 / F::new(2.0);
    let t670 = t337 + t359 + t642 - t645 - t648;
    (t657, t667, t670)
}
