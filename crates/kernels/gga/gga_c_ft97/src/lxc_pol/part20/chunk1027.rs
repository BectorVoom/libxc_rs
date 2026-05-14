//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1027/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1027<F: Float>(t25110: F, t444: F, t4113: F, t24378: F, t25077: F, t25079: F, t6249: F, t6250: F, t96535: F, t24330: F, t25112: F, t25113: F, t43585: F, t6: F, t8: F, t25076: F, t2691: F) -> (F, F, F, F, F, F, F) {
    let t98519 = t25110 * t444;
    let t98520 = t4113 * t98519;
    let t98527 = t25077 * t24378 * t25079;
    let t98530 = t6249 * t96535 * t6250;
    let t98535 = t25112 * t24330 * t25113;
    let t98539 = t4113 * t43585 * t6 * t8;
    let t98544 = t2691 * t25076;
    (t98519, t98520, t98527, t98530, t98535, t98539, t98544)
}
