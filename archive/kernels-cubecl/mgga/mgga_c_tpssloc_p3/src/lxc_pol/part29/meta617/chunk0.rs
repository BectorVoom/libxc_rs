//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2058/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2058<F: Float>(t3545: F, t7372: F, t7378: F, t24698: F, t7327: F, t2121: F, t3427: F, t7381: F, t24574: F, t24795: F, t24799: F, t3590: F, t477: F) -> (F, F, F, F, F, F, F) {
    let t85917 = t7372 * t3545;
    let t85918 = t85917 * t7378;
    let t85920 = t24698 * t7327;
    let t85941 = t2121 * t3427 * t7381;
    let t85943 = t24574 * t24795;
    let t85945 = t24574 * t24799;
    let t85947 = t477 * t3590;
    (t85917, t85918, t85920, t85941, t85943, t85945, t85947)
}
