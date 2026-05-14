//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 711/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk711<F: Float>(t12460: F, t4949: F, t11: F, t12339: F, t4957: F, t1758: F, t2560: F, t3346: F, t1663: F, t571: F, t2554: F, t12345: F, t572: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12461 = t4949 * t12460;
    let t12462 = t11 * t12461;
    let t12464 = t4957 * t12339;
    let t12465 = t1758 * t12464;
    let t12466 = t11 * t12465;
    let t12468 = t2560 * t3346;
    let t12469 = t1758 * t12468;
    let t12470 = t11 * t12469;
    let t12472 = t1663 * t12339;
    let t12473 = t571 * t12472;
    let t12474 = t11 * t12473;
    let t12476 = t2554 * t3346;
    let t12477 = t571 * t12476;
    let t12478 = t11 * t12477;
    let t12480 = t572 * t12345;
    let t12481 = t571 * t12480;
    (t12461, t12462, t12464, t12465, t12466, t12468, t12469, t12470, t12472, t12473, t12474, t12476, t12477, t12478, t12480, t12481)
}
