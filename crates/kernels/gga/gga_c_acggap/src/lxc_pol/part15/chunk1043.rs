//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1043/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1043<F: Float>(t1662: F, t1679: F, t8040: F, t9461: F, t1298: F, t694: F, t8034: F, t2147: F, t2394: F, t7885: F, t864: F, t315: F, t5386: F, t634: F) -> (F, F, F, F, F) {
    let t36769 = F::cast_from(2.0_f64) * t1679 * t8040 * t1662;
    let t36771 = F::cast_from(4.0_f64) * t1679 * t9461;
    let t36774 = F::cast_from(6.0_f64) * t694 * t8034 * t1298;
    let t36794 = t7885 * t2147 * t2394 * t864;
    let t36808 = F::cast_from(0.26341796731742046394e1_f64) * t315 * t634 * t5386;
    (t36769, t36771, t36774, t36794, t36808)
}
