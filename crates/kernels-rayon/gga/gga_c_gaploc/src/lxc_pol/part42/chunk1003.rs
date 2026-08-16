//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 1003/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk1003(t14266: f64, t1564: f64, t12000: f64, t14306: f64, t1445: f64, t1562: f64, t1580: f64, t2778: f64, t3689: f64, t46577: f64, t46580: f64, t46583: f64, t46584: f64, t46587: f64, t46590: f64, t46593: f64, t46596: f64, t46598: f64, t46604: f64, t46605: f64, t475: f64, t48081: f64, t49862: f64, t49866: f64, t49917: f64, t50596: f64, t567: f64, t574: f64, t597: f64, t6710: f64, t6711: f64, t7980: f64) -> f64 {
    let t50693 = t1564 * t14266;
    let t50717 = -0.23005755572352449806e2_f64 * t6710 * t6711 * t50596 - t46577 + t46580 + t46583 - t46584 - t46587 + t46590 - t46593 - t46596 + t46598 - t46604 - t46605 - 0.76685851907841499354e0_f64 * t48081 - 0.69017266717057349418e1_f64 * t1562 * t1445 * t50693 * t475 - 0.92023022289409799224e1_f64 * t574 * t1445 * t7980 * t3689 - 0.92023022289409799224e1_f64 * t574 * t1445 * t2778 * t12000 + 0.23005755572352449806e2_f64 * t1580 * t14306 + 0.23005755572352449806e2_f64 * t597 * t1445 * t49862 + 0.23005755572352449806e2_f64 * t597 * t1445 * t49866 + 0.23005755572352449806e1_f64 * t567 * t1445 * t49917;
    t50717
}
