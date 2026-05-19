//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1061/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1061<F: Float>(t513: F, t5137: F, t1485: F, t1531: F, t1639: F, t466: F, t5152: F, t1532: F, t1661: F, t49: F, t4868: F, t4871: F) -> (F, F, F, F, F) {
    let t16571 = t5137 * t513;
    let t16575 = F::cast_from(0.86748650402413918736e-1_f64) * t1531 * t1485 * t1639;
    let t16578 = F::cast_from(0.38025319932552508021e2_f64) * t1531 * t466 * t5152;
    let t16580 = t1661 * t49 * t1532;
    let t16582 = t4871 * t4868;
    (t16571, t16575, t16578, t16580, t16582)
}
