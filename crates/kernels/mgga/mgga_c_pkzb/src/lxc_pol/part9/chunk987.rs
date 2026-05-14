//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 987/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk987<F: Float>(t16556: F, t1506: F, t16540: F, t555: F, t114: F, t5119: F, t557: F, t1518: F, t1527: F, t1599: F, t513: F, t5137: F, t1485: F, t1531: F, t1639: F, t466: F, t5152: F) -> (F, F, F, F, F, F, F, F) {
    let t16557 = 1.0 / t16556;
    let t16559 = t1506 * t1506;
    let t16560 = 1.0 / t16559;
    let t16563 = 0.91082604192152556044e5 * t555 * t16557 * t16540 * t16560;
    let t16565 = t5119 * t114 * t557;
    let t16569 = 36.0 * t1599 * t1518 * t1527;
    let t16571 = t5137 * t513;
    let t16575 = 0.86748650402413918736e-1 * t1531 * t1485 * t1639;
    let t16578 = 0.38025319932552508021e2 * t1531 * t466 * t5152;
    (t16557, t16560, t16563, t16565, t16569, t16571, t16575, t16578)
}
