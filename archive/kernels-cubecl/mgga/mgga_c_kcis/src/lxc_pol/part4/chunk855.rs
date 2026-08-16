//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 855/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk855<F: Float>(t1489: F, t5632: F, t1468: F, t1464: F, t1307: F, t2046: F, t4170: F, t4160: F, t1650: F, t4163: F, t4162: F, t1497: F) -> (F, F, F, F, F, F, F, F) {
    let t5633 = t5632 * t1489;
    let t5634 = t1468 * t5633;
    let t5635 = t1464 * t5634;
    let t5637 = t2046 * t1307;
    let t5638 = t4170 * t5637;
    let t5639 = t4160 * t5638;
    let t5643 = t1650 * t1489;
    let t5644 = t4163 * t5643;
    let t5645 = t4162 * t5644;
    let t5646 = t4160 * t5645;
    let t5648 = t1650 * t1497;
    (t5633, t5634, t5635, t5638, t5639, t5645, t5646, t5648)
}
