//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 983/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk983<F: Float>(t46595: F, t2392: F, t46499: F, t1: F, t1457: F, t1559: F, t2417: F, t3516: F, t544: F, t42202: F, t42226: F, t13386: F, t1429: F, t549: F) -> (F, F, F, F, F, F) {
    let t46596 = F::new(0.59584149919750711116e-1) * t46595;
    let t46598 = F::new(0.17875244975925213335e2) * t46499 * t2392;
    let t46604 = F::new(0.21450293971110256001e2) * t544 * t1559 * t3516 * t1 * t1457 * t2417;
    let t46605 = F::new(0.25561950635947166451e0) * t42202;
    let t46606 = F::new(0.23005755572352449806e1) * t42226;
    let t46608 = t1429 * t549 * t13386;
    (t46596, t46598, t46604, t46605, t46606, t46608)
}
