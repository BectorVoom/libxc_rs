//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2499/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2499<F: Float>(t1070: F, t193: F, t336: F, t69335: F, t69337: F, t69340: F, t69343: F, t69346: F, t69350: F, t69353: F, t69357: F, t69469: F, t69471: F, t69860: F, t70985: F, t71015: F, t71049: F) -> F {
    let t71055 = t69335 - t69337 - t69340 - t69343 - t69346 + t69350 + t69353 + t69357 - t69469 - t69471 + t193 * t336 * (t69860 + t70985 + t71015 + t71049) * t1070;
    t71055
}
