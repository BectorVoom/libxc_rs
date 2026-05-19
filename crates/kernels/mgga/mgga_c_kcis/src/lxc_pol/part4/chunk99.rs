//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 99/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk99<F: Float>(t287: F, t291: F, t286: F, t285: F) -> (F, F, F, F, F) {
    let t292 = t287 * t291;
    let t293 = t286 * t292;
    let t296 = F::new(1.0) + t285 * t293 / F::new(96.0);
    let t297 = F::ln(t296);
    let t299 = F::new(1.0) + F::new(0.66725e-1) * t297;
    let t300 = F::new(1.0) / t299;
    (t292, t293, t296, t299, t300)
}
