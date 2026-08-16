//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2615/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2615<F: Float>(t1090: F, t11665: F, t1227: F, t15438: F, t15498: F, t15507: F, t15740: F, t1748: F, t19016: F, t19062: F, t19072: F, t22288: F, t22307: F, t3578: F, t45112: F, t45197: F, t4582: F, t4984: F, t4987: F, t6207: F, t65706: F, t65709: F, t66360: F, t66363: F, t66398: F, t70321: F) -> F {
    let t72996 = -t15438 * t19072 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1227 * t4582 * t4987 * t70321 - t11665 * t22288 / F::cast_from(768.0_f64) - t66360 / F::cast_from(1152.0_f64) - t66363 / F::cast_from(2304.0_f64) - t45197 * t3578 * t22307 * t1090 / F::cast_from(768.0_f64) - t45112 + t65709 * t1748 / F::cast_from(144.0_f64) + t15507 * t19062 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t15740 * t19016 + t65706 * t4984 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t66398 + t15498 * t6207 / F::cast_from(288.0_f64);
    t72996
}
