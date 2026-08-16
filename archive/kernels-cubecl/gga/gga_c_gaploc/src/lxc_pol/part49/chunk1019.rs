//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1019/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1019<F: Float>(t43476: F, t13038: F, t2194: F, t313: F, t3470: F, t43246: F, t10789: F, t769: F, t10627: F, t2530: F) -> (F, F, F, F, F) {
    let t43477 = F::cast_from(0.31952438294933958064e-1_f64) * t43476;
    let t43479 = F::cast_from(0.92023022289409799224e1_f64) * t2194 * t13038;
    let t43481 = t313 * t43246 * t3470;
    let t43484 = t769 * t10789 * t3470;
    let t43486 = t10627 * t2530;
    (t43477, t43479, t43481, t43484, t43486)
}
