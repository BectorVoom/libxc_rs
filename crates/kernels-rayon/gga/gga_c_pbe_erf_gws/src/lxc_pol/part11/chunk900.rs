//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 900/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk900(t5: f64, t922: f64, t168: f64, t270: f64, t153: f64, t18046: f64, t274: f64, t1383: f64, t762: f64, t4598: f64, t528: f64, t1: f64, t119: f64, t1372: f64, t1375: f64, t1379: f64, t1380: f64, t159: f64, t161: f64, t16451: f64, t18032: f64, t18050: f64, t20: f64, t3: f64, t39: f64, t413: f64, t4573: f64, t4580: f64, t4586: f64, t4589: f64, t4592: f64, t545: f64, t5589: f64, t6045: f64, t696: f64, t697: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18344 = t5 * t922;
    let t18347 = 0.90790602394455990432e0_f64 * t168 * t18344 * t270;
    let t18359 = 0.19192636997366703204e2_f64 * t153 * t18046 * t274;
    let t18363 = 0.10051538464260528225e1_f64 * t762 * t1383;
    let t18372 = 0.33505128214201760751e0_f64 * t528 * t4598;
    let t18411 = t18050 / 2.0_f64 + 0.1254e0_f64 * t18032 * t3 * t697 - 0.2508e0_f64 * t4580 * t1375 + 0.4717548e-1_f64 * t16451 * t20 * t1380 + 0.39013333333333333333e0_f64 * t1372 * t4589 - 0.12580128e0_f64 * t4586 * t4592 + 0.75322371094039916836e-2_f64 * t545 * t39 * t161 - 0.32511111111111111111e0_f64 * t696 * t4573 * t161 + 0.1397792e0_f64 * t1379 * t5589 * t161 - 0.15064474218807983367e-1_f64 * t159 * t413 * t161 + 0.11806781668990756964e-3_f64 * t159 * t6045 * t1 * t119 * t161;
    (t18344, t18347, t18359, t18363, t18372, t18411)
}
