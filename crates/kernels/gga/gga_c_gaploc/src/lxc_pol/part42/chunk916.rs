//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 916/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk916<F: Float>(t1457: F, t44474: F, t4540: F, t13468: F, t21370: F, t1445: F, t44480: F, t597: F, t13368: F, t4953: F, t11172: F, t2293: F) -> (F, F, F, F, F) {
    let t46450 = F::cast_from(0.21450293971110256001e1_f64) * t4540 * t1457 * t44474;
    let t46457 = t21370 * t13468;
    let t46461 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t1445 * t44480;
    let t46463 = F::cast_from(0.62115540045351614476e2_f64) * t4953 * t13368;
    let t46471 = F::cast_from(0.43710935587469654631e2_f64) * t597 * t1445 * t11172 * t2293;
    (t46450, t46457, t46461, t46463, t46471)
}
