//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 1003/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk1003<F: Float>(t14266: F, t1564: F, t12000: F, t14306: F, t1445: F, t1562: F, t1580: F, t2778: F, t3689: F, t46577: F, t46580: F, t46583: F, t46584: F, t46587: F, t46590: F, t46593: F, t46596: F, t46598: F, t46604: F, t46605: F, t475: F, t48081: F, t49862: F, t49866: F, t49917: F, t50596: F, t567: F, t574: F, t597: F, t6710: F, t6711: F, t7980: F) -> F {
    let t50693 = t1564 * t14266;
    let t50717 = -F::new(0.23005755572352449806e2) * t6710 * t6711 * t50596 - t46577 + t46580 + t46583 - t46584 - t46587 + t46590 - t46593 - t46596 + t46598 - t46604 - t46605 - F::new(0.76685851907841499354e0) * t48081 - F::new(0.69017266717057349418e1) * t1562 * t1445 * t50693 * t475 - F::new(0.92023022289409799224e1) * t574 * t1445 * t7980 * t3689 - F::new(0.92023022289409799224e1) * t574 * t1445 * t2778 * t12000 + F::new(0.23005755572352449806e2) * t1580 * t14306 + F::new(0.23005755572352449806e2) * t597 * t1445 * t49862 + F::new(0.23005755572352449806e2) * t597 * t1445 * t49866 + F::new(0.23005755572352449806e1) * t567 * t1445 * t49917;
    t50717
}
