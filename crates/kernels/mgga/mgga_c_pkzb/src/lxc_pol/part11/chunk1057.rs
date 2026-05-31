//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1057/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1057<F: Float>(t16559: F, t16540: F, t16557: F, t555: F, t1518: F, t1527: F, t1599: F, t513: F, t5137: F, t1485: F, t1531: F, t1639: F) -> (F, F, F, F, F) {
    let t16560 = F::cast_from(1.0_f64) / t16559;
    let t16563 = F::cast_from(0.91082604192152556044e5_f64) * t555 * t16557 * t16540 * t16560;
    let t16569 = F::cast_from(36.0_f64) * t1599 * t1518 * t1527;
    let t16571 = t5137 * t513;
    let t16575 = F::cast_from(0.86748650402413918736e-1_f64) * t1531 * t1485 * t1639;
    (t16560, t16563, t16569, t16571, t16575)
}
