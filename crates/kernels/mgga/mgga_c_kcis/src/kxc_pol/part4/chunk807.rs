//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 807/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk807<F: Float>(t1498: F, t5752: F, t1464: F, t1494: F, t1928: F, t1497: F, t1395: F, t2012: F, t3738: F, t2013: F, t3728: F, t2003: F, t1307: F, t5632: F, t1394: F, t1397: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5753 = t5752 * t1498;
    let t5754 = t1464 * t5753;
    let t5756 = t1928 * t1494;
    let t5757 = t5756 * t1497;
    let t5758 = t1395 * t5757;
    let t5759 = t1464 * t5758;
    let t5761 = t3738 * t2012;
    let t5762 = t1464 * t5761;
    let t5764 = t3728 * t2013;
    let t5766 = t3728 * t2003;
    let t5769 = t5632 * t1307;
    let t5770 = t1395 * t5769;
    let t5771 = t1394 * t5770;
    let t5773 = t5752 * t1397;
    (t5753, t5754, t5756, t5757, t5758, t5759, t5761, t5762, t5764, t5766, t5769, t5770, t5771, t5773)
}
