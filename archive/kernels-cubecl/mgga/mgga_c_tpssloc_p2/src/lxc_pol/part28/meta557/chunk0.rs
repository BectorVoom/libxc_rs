//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1828/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1828<F: Float>(t81688: F, t81716: F, t82046: F, t82122: F, t225: F, t24200: F, t82153: F, t82218: F, t24237: F, t24235: F, t2105: F, t3931: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t84995 = F::cast_from(0.27415567780803773942e-2_f64) * t81688;
    let t85003 = F::cast_from(0.19739208802178717238e0_f64) * t81716;
    let t85027 = F::cast_from(0.55440370401180965083e0_f64) * t82046;
    let t85060 = F::cast_from(0.3244175520728446583e0_f64) * t82122;
    let t85079 = t24200 * t225;
    let t85101 = F::cast_from(0.27415567780803773942e-2_f64) * t82153;
    let t85129 = F::cast_from(0.55440370401180965083e0_f64) * t82218;
    let t85146 = t24237 * t225;
    let t85152 = t24235 * t225;
    let t85379 = t3931 * t2105;
    (t84995, t85003, t85027, t85060, t85079, t85101, t85129, t85146, t85152, t85379)
}
