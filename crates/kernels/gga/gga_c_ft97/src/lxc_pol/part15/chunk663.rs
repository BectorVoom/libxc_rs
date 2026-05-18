//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 663/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk663<F: Float>(t19100: F, t4092: F, t39: F, t817: F, t1200: F, t800: F, t285: F, t5249: F, t8959: F, t4939: F, t703: F, t1196: F, t284: F) -> (F, F, F, F, F, F, F, F) {
    let t19101 = t4092 * t19100;
    let t19106 = t817 * t39;
    let t19107 = t1200 * t19106;
    let t19132 = t800 * t19100;
    let t19135 = t285 * t19106;
    let t19167 = F::new(0.8854768453090786061e-3) * t8959 * t5249;
    let t19168 = t703 * t4939;
    let t19233 = t1196 * t284;
    (t19101, t19106, t19107, t19132, t19135, t19167, t19168, t19233)
}
