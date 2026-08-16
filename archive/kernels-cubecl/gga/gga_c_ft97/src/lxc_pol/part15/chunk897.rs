//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 897/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk897<F: Float>(t1160: F, t9802: F, t9895: F, t1087: F, t3704: F, t89: F, t1611: F, t806: F, t1609: F, t9523: F, t1092: F, t3051: F) -> (F, F, F, F, F, F) {
    let t51990 = t9802 * t1160;
    let t52006 = t9895 * t1160;
    let t52212 = t89 * t3704 * t1087;
    let t52324 = t1611 * t806;
    let t52358 = t1609 * t9523;
    let t52453 = t3051 * t1092;
    (t51990, t52006, t52212, t52324, t52358, t52453)
}
