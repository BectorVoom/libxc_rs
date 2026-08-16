//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1047/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1047<F: Float>(t24046: F, t24062: F, t539: F, t22645: F, t225: F, t7192: F, t2091: F, t3887: F, t3911: F, t12021: F, t3888: F, t7179: F) -> (F, F, F, F, F, F, F) {
    let t24063 = t24046 + t24062;
    let t24064 = t539 * t24063;
    let t24071 = F::cast_from(0.16449340668482264365e-1_f64) * t22645;
    let t24082 = t7192 * t225;
    let t24088 = t3887 * t2091 * t3911;
    let t24092 = t12021 * t2091 * t3888;
    let t24095 = t7179 * t225;
    (t24063, t24064, t24071, t24082, t24088, t24092, t24095)
}
