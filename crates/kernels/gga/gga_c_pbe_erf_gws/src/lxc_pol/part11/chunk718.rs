//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 718/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk718<F: Float>(t12559: F, t1885: F, t1820: F, t12350: F, t5003: F, t1640: F, t639: F, t1010: F, t10848: F, t7122: F, t10329: F, t12339: F, t1664: F, t590: F, t587: F, t1643: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12560 = t1885 * t12559;
    let t12562 = 8.0 / 5.0 * t1820 * t12560;
    let t12563 = t5003 * t12350;
    let t12564 = t1640 * t12563;
    let t12566 = 8.0 / 9.0 * t639 * t12564;
    let t12568 = 4.0 / 15.0 * t10848 * t1010;
    let t12569 = 4.0 / 45.0 * t7122;
    let t12570 = 16.0 / 15.0 * t10329;
    let t12571 = t1664 * t12339;
    let t12572 = t590 * t12571;
    let t12574 = 8.0 / 15.0 * t587 * t12572;
    let t12575 = t1643 * t12350;
    (t12560, t12562, t12563, t12564, t12566, t12568, t12569, t12570, t12571, t12572, t12574, t12575)
}
