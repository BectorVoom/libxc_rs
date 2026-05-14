//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1307/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1307<F: Float>(t114751: F, t1248: F, t31936: F, t44351: F, t10697: F, t5309: F, t6386: F, t1091: F, t111667: F, t111682: F, t111685: F, t111687: F, t111705: F, t111716: F, t111732: F, t111733: F, t111737: F, t1253: F, t1466: F, t18123: F, t19002: F, t19006: F, t193: F, t2665: F, t4129: F, t6216: F, t6217: F, t6222: F) -> (F, F, F, F) {
    let t125534 = t114751 * t1248;
    let t125541 = t44351 * t31936;
    let t125544 = t10697 * t6386 * t5309;
    let t125553 = -t111667 - t6216 * t2665 * t6217 * t18123 / 18.0 - 2.0 / 3.0 * t1466 * t193 * t6222 * t1253 * t4129 - 8.0 / 27.0 * t111682 - 4.0 * t125534 + t111685 - 2.0 / 27.0 * t111687 - t6216 * t2665 * t111716 * t1091 / 9.0 - 12.0 * t125541 - 12.0 * t125544 - 4.0 / 9.0 * t111732 * t111733 * t19002 + 4.0 / 27.0 * t111732 * t111737 * t19006 - 4.0 / 81.0 * t111705;
    (t125534, t125541, t125544, t125553)
}
