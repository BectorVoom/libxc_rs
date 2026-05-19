//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1012/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1012<F: Float>(t4551: F, t700: F, t1383: F, t1597: F, t4598: F, t528: F, t4563: F, t1602: F, t536: F, t1: F, t119: F, t1372: F, t1375: F, t1379: F, t1380: F, t159: F, t161: F, t16451: F, t18032: F, t18050: F, t20: F, t3: F, t39: F, t413: F, t4573: F, t4580: F, t4586: F, t4589: F, t4592: F, t545: F, t5589: F, t6045: F, t696: F, t697: F) -> (F, F, F, F, F, F, F) {
    let t18367 = t4551 * t700;
    let t18369 = t1597 * t1383;
    let t18372 = F::cast_from(0.33505128214201760751e0_f64) * t528 * t4598;
    let t18375 = t4563 * t700;
    let t18377 = t1602 * t1383;
    let t18379 = t536 * t4598;
    let t18411 = t18050 / F::new(2.0) + F::new(0.1254e0) * t18032 * t3 * t697 - F::new(0.2508e0) * t4580 * t1375 + F::new(0.4717548e-1) * t16451 * t20 * t1380 + F::cast_from(0.39013333333333333333e0_f64) * t1372 * t4589 - F::new(0.12580128e0) * t4586 * t4592 + F::cast_from(0.75322371094039916836e-2_f64) * t545 * t39 * t161 - F::cast_from(0.32511111111111111111e0_f64) * t696 * t4573 * t161 + F::new(0.1397792e0) * t1379 * t5589 * t161 - F::cast_from(0.15064474218807983367e-1_f64) * t159 * t413 * t161 + F::cast_from(0.11806781668990756964e-3_f64) * t159 * t6045 * t1 * t119 * t161;
    (t18367, t18369, t18372, t18375, t18377, t18379, t18411)
}
