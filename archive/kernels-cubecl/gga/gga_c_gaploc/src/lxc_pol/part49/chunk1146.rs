//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1146/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1146<F: Float>(t13866: F, t5782: F, t1445: F, t2087: F, t39027: F, t935: F, t12218: F, t2530: F, t13862: F, t2197: F, t47220: F, t833: F) -> (F, F, F, F, F) {
    let t47527 = t5782 * t13866;
    let t47531 = t2087 * t1445 * t39027 * t935;
    let t47535 = t2087 * t1445 * t12218 * t2530;
    let t47537 = t2197 * t13862;
    let t47540 = t833 * t1445 * t47220;
    (t47527, t47531, t47535, t47537, t47540)
}
