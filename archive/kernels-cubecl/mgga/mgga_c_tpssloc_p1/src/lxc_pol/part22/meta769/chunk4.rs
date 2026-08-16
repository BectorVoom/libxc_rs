//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2614/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2614<F: Float>(t11697: F, t22161: F, t3577: F, t19025: F, t5001: F, t1090: F, t11668: F, t1174: F, t1218: F, t15569: F, t15594: F, t1735: F, t18215: F, t18368: F, t18590: F, t18969: F, t22299: F, t3578: F, t44621: F, t45044: F, t45119: F, t5024: F, t52628: F, t53162: F, t6211: F, t66334: F, t66337: F, t71164: F) -> F {
    let t72959 = t3577 * t11697 * t22161;
    let t72967 = t5001 * t19025;
    let t72970 = F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t66334 - t66337 / F::cast_from(1152.0_f64) - t45119 * t3578 * t22299 * t1090 / F::cast_from(4608.0_f64) - F::cast_from(5.0_f64) / F::cast_from(3888.0_f64) * t45044 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1174 * t44621 * t71164 - t15594 * t6211 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3577 * t11668 * t1735 * t18215 - t72959 / F::cast_from(2304.0_f64) + t53162 + t5024 * t18590 / F::cast_from(72.0_f64) + t15569 * t18969 / F::cast_from(288.0_f64) + t52628 * t18368 / F::cast_from(144.0_f64) + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t72967 * t1218;
    t72970
}
