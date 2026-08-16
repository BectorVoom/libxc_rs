//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 886/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk886<F: Float>(t16576: F, t1332: F, t35: F, t226: F, t7: F, t7236: F, t7271: F, t4991: F, t597: F, t5210: F, t735: F, t174: F, t177: F, t2200: F) -> (F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t16577 = F::cast_from(192.0_f64) * t16576;
    let t16578 = t35 * t1332;
    let t16579 = F::cast_from(120.0_f64) * t16578;
    let t16595 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t226 * (-F::cast_from(0.42777777777777777777e1_f64) * t7271 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t7236) * pi * t7;
    let t16621 = t4991 * t597;
    let t16666 = t5210 * t735;
    let t16704 = t174 * t2200 * t177;
    (t16577, t16578, t16579, t16595, t16621, t16666, t16704)
}
