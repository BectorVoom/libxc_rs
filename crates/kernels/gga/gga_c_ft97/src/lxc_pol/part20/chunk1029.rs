//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1029/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1029<F: Float>(t24330: F, t25049: F, t25050: F, t25111: F, t2691: F, t25120: F, t6051: F, t1472: F, t96737: F, t6256: F, t96725: F, t54863: F, t6241: F, t96558: F, t96722: F, t96536: F) -> (F, F, F, F, F, F, F, F, F) {
    let t98561 = t25049 * t24330 * t25050;
    let t98563 = t2691 * t25111;
    let t98570 = t25120 * t6051;
    let t98581 = 0.18521666970164609055e-1 * t1472 * t96737;
    let t98589 = t6256 * t96725;
    let t98593 = t54863 * t6241;
    let t98598 = t6256 * t96558;
    let t98600 = t6256 * t96722;
    let t98612 = t6256 * t96536;
    (t98561, t98563, t98570, t98581, t98589, t98593, t98598, t98600, t98612)
}
