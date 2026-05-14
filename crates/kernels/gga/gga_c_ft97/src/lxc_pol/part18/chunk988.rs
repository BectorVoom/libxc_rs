//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 988/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk988<F: Float>(t26373: F, t3219: F, t26372: F, t3266: F, t5717: F, t11810: F, t23249: F, t3271: F, t11490: F, t26052: F, t83: F, t432: F, t6557: F, t452: F, t488: F, t25599: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26374 = t26373 * t3219;
    let t26375 = t26372 * t26374;
    let t26378 = t5717 * t3266;
    let t26379 = t11810 * t26378;
    let t26382 = t23249 * t3271;
    let t26383 = t11490 * t26382;
    let t26387 = t83 * t26052;
    let t26390 = t6557 * t432;
    let t26392 = t452 * t488 * t26390;
    let t26395 = t83 * t25599;
    (t26374, t26375, t26378, t26379, t26382, t26383, t26387, t26390, t26392, t26395)
}
