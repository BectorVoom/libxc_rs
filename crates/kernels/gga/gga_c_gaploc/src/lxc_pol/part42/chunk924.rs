//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 924/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk924<F: Float>(t1: F, t1457: F, t1559: F, t2417: F, t3516: F, t544: F, t42202: F, t42226: F, t13386: F, t1429: F, t549: F, t13261: F, t4614: F, t597: F) -> (F, F, F, F, F) {
    let t46604 = F::new(0.21450293971110256001e2) * t544 * t1559 * t3516 * t1 * t1457 * t2417;
    let t46605 = F::new(0.25561950635947166451e0) * t42202;
    let t46606 = F::new(0.23005755572352449806e1) * t42226;
    let t46608 = t1429 * t549 * t13386;
    let t46609 = F::new(0.29792074959875355558e-1) * t46608;
    let t46612 = F::new(0.15337170381568299871e2) * t597 * t4614 * t13261;
    (t46604, t46605, t46606, t46609, t46612)
}
