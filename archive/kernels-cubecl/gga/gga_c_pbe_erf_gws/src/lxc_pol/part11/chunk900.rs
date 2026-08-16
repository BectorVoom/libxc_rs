//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 900/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk900<F: Float>(t5: F, t922: F, t168: F, t270: F, t153: F, t18046: F, t274: F, t1383: F, t762: F, t4598: F, t528: F, t1: F, t119: F, t1372: F, t1375: F, t1379: F, t1380: F, t159: F, t161: F, t16451: F, t18032: F, t18050: F, t20: F, t3: F, t39: F, t413: F, t4573: F, t4580: F, t4586: F, t4589: F, t4592: F, t545: F, t5589: F, t6045: F, t696: F, t697: F) -> (F, F, F, F, F, F) {
    let t18344 = t5 * t922;
    let t18347 = F::cast_from(0.90790602394455990432e0_f64) * t168 * t18344 * t270;
    let t18359 = F::cast_from(0.19192636997366703204e2_f64) * t153 * t18046 * t274;
    let t18363 = F::cast_from(0.10051538464260528225e1_f64) * t762 * t1383;
    let t18372 = F::cast_from(0.33505128214201760751e0_f64) * t528 * t4598;
    let t18411 = t18050 / F::cast_from(2.0_f64) + F::cast_from(0.1254e0_f64) * t18032 * t3 * t697 - F::cast_from(0.2508e0_f64) * t4580 * t1375 + F::cast_from(0.4717548e-1_f64) * t16451 * t20 * t1380 + F::cast_from(0.39013333333333333333e0_f64) * t1372 * t4589 - F::cast_from(0.12580128e0_f64) * t4586 * t4592 + F::cast_from(0.75322371094039916836e-2_f64) * t545 * t39 * t161 - F::cast_from(0.32511111111111111111e0_f64) * t696 * t4573 * t161 + F::cast_from(0.1397792e0_f64) * t1379 * t5589 * t161 - F::cast_from(0.15064474218807983367e-1_f64) * t159 * t413 * t161 + F::cast_from(0.11806781668990756964e-3_f64) * t159 * t6045 * t1 * t119 * t161;
    (t18344, t18347, t18359, t18363, t18372, t18411)
}
