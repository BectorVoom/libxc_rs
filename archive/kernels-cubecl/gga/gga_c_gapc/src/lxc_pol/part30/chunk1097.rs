//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1097/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1097<F: Float>(t11781: F, t9485: F, t19179: F, t3792: F, t30167: F, t33202: F, t11997: F, t2778: F, t11937: F, t9999: F, t16182: F, t29033: F) -> (F, F, F, F, F, F, F) {
    let t33462 = t11781 * t9485;
    let t33464 = t3792 * t19179;
    let t33466 = t33202 * t30167;
    let t33468 = t11997 * t2778;
    let t33470 = t11937 * t2778;
    let t33472 = t11781 * t9999;
    let t33474 = t29033 * t16182;
    (t33462, t33464, t33466, t33468, t33470, t33472, t33474)
}
