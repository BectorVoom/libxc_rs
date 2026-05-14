//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 845/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk845<F: Float>(t42199: F, t204: F, t46254: F, t587: F, t2487: F, t6711: F, t46103: F, t6710: F, t13392: F, t4391: F, t549: F, t2392: F, t46499: F, t1: F, t1457: F, t1559: F, t2417: F, t3516: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t46584 = 0.23005755572352449806e1 * t42199;
    let t46587 = 0.92023022289409799224e1 * t587 * t204 * t46254;
    let t46590 = 0.43710935587469654631e2 * t2487 * t6711 * t46254;
    let t46593 = 0.11502877786176224903e2 * t6710 * t6711 * t46103;
    let t46595 = t4391 * t549 * t13392;
    let t46596 = 0.59584149919750711116e-1 * t46595;
    let t46598 = 0.17875244975925213335e2 * t46499 * t2392;
    let t46604 = 0.21450293971110256001e2 * t544 * t1559 * t3516 * t1 * t1457 * t2417;
    (t46584, t46587, t46590, t46593, t46596, t46598, t46604)
}
