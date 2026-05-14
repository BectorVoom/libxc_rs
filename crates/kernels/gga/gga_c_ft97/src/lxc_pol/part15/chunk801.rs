//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 801/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk801<F: Float>(t1140: F, t3281: F, t1144: F, t1162: F, t89: F, t9555: F, t1170: F, t1160: F, t9802: F, t9895: F, t1087: F, t3704: F, t1611: F, t806: F, t1609: F, t9523: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51355 = t3281 * t1140;
    let t51453 = t3281 * t1144;
    let t51882 = t89 * t9555 * t1162;
    let t51972 = t3281 * t1170;
    let t51990 = t9802 * t1160;
    let t52006 = t9895 * t1160;
    let t52212 = t89 * t3704 * t1087;
    let t52324 = t1611 * t806;
    let t52358 = t1609 * t9523;
    (t51355, t51453, t51882, t51972, t51990, t52006, t52212, t52324, t52358)
}
