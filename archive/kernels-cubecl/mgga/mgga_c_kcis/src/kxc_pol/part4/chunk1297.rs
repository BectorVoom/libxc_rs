//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1297/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1297<F: Float>(t1307: F, t5875: F, t16633: F, t4160: F, t5880: F, t12281: F, t5671: F, t4129: F, t5756: F, t1468: F, t1464: F, t2011: F, t3954: F) -> (F, F, F, F, F) {
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
    let t16650 = t1468 * t16649;
    let t16651 = t1464 * t16650;
    let t16653 = t2011 * t3954;
    (t16636, t16640, t16644, t16651, t16653)
}
