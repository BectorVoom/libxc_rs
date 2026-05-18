//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 998/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk998<F: Float>(t13728: F, t4614: F, t597: F, t1445: F, t46915: F, t574: F, t1: F, t106: F, t13749: F, t192: F, t536: F, t40192: F) -> (F, F, F, F) {
    let t47902 = t597 * t4614 * t13728;
    let t47912 = F::new(0.46011511144704899612e1) * t574 * t1445 * t46915;
    let t47918 = t13749 * t1 * t106 * t192;
    let t47920 = F::new(0.35750489951850426669e0) * t536 * t47918;
    let t47925 = F::new(0.38342925953920749677e0) * t40192;
    (t47902, t47912, t47920, t47925)
}
