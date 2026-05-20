//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3031/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3031<F: Float>(t15654: F, t3286: F, t16543: F, t3046: F, t1071: F, t1087: F, t1089: F, t12133: F, t12146: F, t12154: F, t12160: F, t15780: F, t16183: F, t16393: F, t16410: F, t16468: F, t16488: F, t16537: F, t16573: F, t16581: F, t19603: F, t3043: F, t3287: F, t3288: F, t43360: F, t4980: F, t4984: F, t4995: F, t4996: F, t4999: F, t55345: F) -> F {
    let t55685 = t15654 * t3286;
    let t55701 = t3046 * t16543;
    let t55711 = -F::cast_from(0.39512695097613069591e1_f64) * t12146 * t16468 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t1071 * t16183 * t1089 + F::cast_from(0.39512695097613069591e1_f64) * t16410 * t16581 - F::cast_from(0.39512695097613069591e1_f64) * t55685 * t3288 - F::cast_from(0.19756347548806534796e1_f64) * t12160 * t16488 - F::cast_from(0.19756347548806534796e1_f64) * t4996 * t15780 * t16573 + F::cast_from(0.39512695097613069591e1_f64) * t3043 * t4980 * t4984 - F::cast_from(0.19756347548806534796e1_f64) * t3043 * t4995 * t4999 - F::cast_from(0.39512695097613069591e1_f64) * t43360 * t16537 - F::cast_from(0.39512695097613069591e1_f64) * t55701 * t3288 - F::cast_from(0.19756347548806534796e1_f64) * t12154 * t16393 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t55345 * t1089 + F::cast_from(0.39512695097613069591e1_f64) * t19603 * t12133;
    t55711
}
