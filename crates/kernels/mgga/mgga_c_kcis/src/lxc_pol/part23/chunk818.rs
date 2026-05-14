//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 818/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk818<F: Float>(t1464: F, t16624: F, t3728: F, t5634: F, t5758: F, t5417: F, t4135: F, t4169: F, t1307: F, t5875: F, t4160: F, t5880: F, t12281: F, t5671: F, t4129: F, t5756: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16625 = t1464 * t16624;
    let t16627 = t3728 * t5634;
    let t16628 = 0.88437037037037037034e-2 * t16627;
    let t16629 = t3728 * t5758;
    let t16631 = t3728 * t5417;
    let t16632 = 0.33163888888888888888e-2 * t16631;
    let t16633 = t4169 * t4135;
    let t16634 = t5875 * t1307;
    let t16635 = t16633 * t16634;
    let t16636 = t4160 * t16635;
    let t16638 = t5880 * t1307;
    let t16639 = t12281 * t16638;
    let t16640 = t4160 * t16639;
    let t16642 = t5671 * t1307;
    let t16643 = t12281 * t16642;
    let t16644 = t4160 * t16643;
    let t16649 = t5756 * t4129;
    (t16625, t16627, t16628, t16629, t16631, t16632, t16634, t16636, t16638, t16640, t16642, t16644, t16649)
}
