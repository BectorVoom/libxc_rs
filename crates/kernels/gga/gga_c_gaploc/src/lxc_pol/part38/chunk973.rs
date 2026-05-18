//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 973/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk973<F: Float>(t10532: F, t10533: F, t46115: F, t13371: F, t4614: F, t574: F, t1457: F, t44474: F, t4540: F, t13468: F, t21370: F, t1445: F, t44480: F, t597: F) -> (F, F, F, F, F) {
    let t46435 = F::new(0.55213813373645879534e2) * t10532 * t10533 * t46115;
    let t46447 = F::new(0.61348681526273199483e1) * t574 * t4614 * t13371;
    let t46450 = F::new(0.21450293971110256001e1) * t4540 * t1457 * t44474;
    let t46457 = t21370 * t13468;
    let t46461 = F::new(0.11502877786176224903e2) * t597 * t1445 * t44480;
    (t46435, t46447, t46450, t46457, t46461)
}
