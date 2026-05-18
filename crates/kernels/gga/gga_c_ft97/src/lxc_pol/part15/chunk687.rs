//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 687/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk687<F: Float>(t20113: F, t7750: F, t27: F, t89: F, t3013: F, t4495: F, t28: F, t11076: F, t15606: F, t15609: F, t15612: F, t15899: F, t20101: F, t20105: F, t20109: F, t8190: F) -> (F, F, F, F, F) {
    let t20114 = t7750 * t20113;
    let t20116 = t89 * t27 * t20114;
    let t20117 = t3013 * t4495;
    let t20119 = t89 * t28 * t20117;
    let t20123 = -t20101 / F::new(6.0) - t20105 / F::new(3.0) - t20109 / F::new(3.0) - t15899 / F::new(9.0) - t8190 - F::new(2.0) / F::new(9.0) * t11076 - t20116 + t20119 - t15609 / F::new(9.0) + t15612 / F::new(18.0) + t15606 / F::new(27.0);
    (t20114, t20116, t20117, t20119, t20123)
}
