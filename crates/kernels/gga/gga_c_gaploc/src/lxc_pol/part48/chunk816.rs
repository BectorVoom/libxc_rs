//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 816/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk816<F: Float>(t41629: F, t11371: F, t2482: F, t9267: F, t2478: F, t3536: F, t6576: F, t41588: F, t41592: F, t41594: F, t41606: F, t41615: F, t41621: F, t41631: F, t41636: F, t41640: F, t41643: F, t41645: F, t46030: F, t46031: F, t46033: F, t46035: F, t46036: F) -> (F,) {
    let t46037 = 0.11916829983950142223e0 * t41629;
    let t46044 = t9267 * t11371 * t2482;
    let t46045 = 0.9585731488480187419e0 * t46044;
    let t46047 = t6576 * t3536 * t2478;
    let t46049 = 0.38342925953920749677e1 * t41588 - 0.23005755572352449806e1 * t41592 - 0.51123901271894332903e1 * t41594 - 0.38342925953920749677e1 * t41606 - t46030 + t46031 + 0.63904876589867916128e-1 * t41615 - t46033 + 0.11916829983950142223e0 * t41621 + t46035 + t46036 + t46037 + 0.76685851907841499353e0 * t41631 + 0.76685851907841499353e0 * t41636 - 0.17041300423964777634e0 * t41640 - 0.59584149919750711116e-1 * t41643 + 0.38342925953920749677e1 * t41645 + t46045 + 0.38342925953920749677e0 * t46047;
    (t46049,)
}
