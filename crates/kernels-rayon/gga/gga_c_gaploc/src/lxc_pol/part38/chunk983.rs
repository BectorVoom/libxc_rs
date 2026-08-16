//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 983/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk983(t46595: f64, t2392: f64, t46499: f64, t1: f64, t1457: f64, t1559: f64, t2417: f64, t3516: f64, t544: f64, t42202: f64, t42226: f64, t13386: f64, t1429: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46596 = 0.59584149919750711116e-1_f64 * t46595;
    let t46598 = 0.17875244975925213335e2_f64 * t46499 * t2392;
    let t46604 = 0.21450293971110256001e2_f64 * t544 * t1559 * t3516 * t1 * t1457 * t2417;
    let t46605 = 0.25561950635947166451e0_f64 * t42202;
    let t46606 = 0.23005755572352449806e1_f64 * t42226;
    let t46608 = t1429 * t549 * t13386;
    (t46596, t46598, t46604, t46605, t46606, t46608)
}
