//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 931/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk931<F: Float>(t37956: F, t895: F, t1445: F, t36117: F, t574: F, t874: F, t11167: F, t2293: F, t13426: F, t18658: F, t44470: F, t597: F) -> (F, F, F, F, F) {
    let t46742 = F::new(0.23833659967900284446e0) * t895 * t37956;
    let t46754 = F::new(0.46011511144704899612e1) * t574 * t1445 * t36117 * t874;
    let t46758 = F::new(0.46011511144704899612e1) * t574 * t1445 * t11167 * t2293;
    let t46760 = F::new(0.21450293971110256001e1) * t18658 * t13426;
    let t46765 = F::new(0.11502877786176224903e2) * t597 * t1445 * t44470;
    (t46742, t46754, t46758, t46760, t46765)
}
