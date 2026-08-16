//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 648/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk648<F: Float>(t3113: F, t5691: F, t8557: F, t11468: F, t25924: F, t1339: F, t1871: F, t3266: F, t3052: F, t447: F, t5750: F, t925: F) -> (F, F, F, F, F, F) {
    let t26318 = t5691 * t3113;
    let t26319 = t8557 * t26318;
    let t26322 = t11468 * t25924;
    let t26326 = t1871 * t1339 * t3266;
    let t26330 = t447 * t1339 * t3052;
    let t26334 = t447 * t5750 * t925;
    (t26318, t26319, t26322, t26326, t26330, t26334)
}
