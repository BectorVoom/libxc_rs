//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2602/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2602<F: Float>(t19051: F, t4993: F, t11784: F, t1227: F, t21762: F, t248: F, t11721: F, t6218: F, t11668: F, t11692: F, t15503: F, t15700: F, t1735: F, t18241: F, t19058: F, t3515: F, t3577: F, t3578: F, t45114: F, t45197: F, t4582: F, t4972: F, t4980: F, t52548: F, t52732: F, t52897: F, t5392: F, t65464: F, t65819: F, t65881: F, t65963: F, t66533: F, t70321: F) -> (F, F) {
    let t72556 = t19051 * t4993;
    let t72560 = t1227 * t248 * t11784 * t21762;
    let t72577 = t6218 * t11721;
    let t72593 = t65819 / F::cast_from(3456.0_f64) - t72556 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t72560 - t15503 * t19058 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t11692 * t11668 * t15700 * t52548 * t5392 - t1227 * t4582 * t4972 * t70321 / F::cast_from(768.0_f64) - t3577 * t3578 * t1735 * t18241 / F::cast_from(1536.0_f64) + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t45197 * t52897 * t72577 * t15700 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t45114 * t52897 * t65464 * t15700 - t52732 - t3515 * t4582 * t66533 * t1735 / F::cast_from(1024.0_f64) - t65881 / F::cast_from(1536.0_f64) + t65963 * t4980 / F::cast_from(512.0_f64);
    (t72577, t72593)
}
