//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 794/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk794<F: Float>(t1457: F, t44474: F, t4540: F, t13468: F, t21370: F, t1445: F, t44480: F, t597: F, t13368: F, t4953: F, t11172: F, t2293: F, t13383: F, t1580: F, t11259: F, t2464: F, t2465: F, t6914: F) -> (F, F, F, F, F, F, F) {
    let t46450 = 0.21450293971110256001e1 * t4540 * t1457 * t44474;
    let t46457 = t21370 * t13468;
    let t46461 = 0.11502877786176224903e2 * t597 * t1445 * t44480;
    let t46463 = 0.62115540045351614476e2 * t4953 * t13368;
    let t46471 = 0.43710935587469654631e2 * t597 * t1445 * t11172 * t2293;
    let t46473 = 0.11502877786176224903e2 * t1580 * t13383;
    let t46480 = t6914 * t2464 * t2465 * t11259;
    (t46450, t46457, t46461, t46463, t46471, t46473, t46480)
}
