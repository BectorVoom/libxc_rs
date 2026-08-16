//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2197/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2197<F: Float>(t18050: F, t18168: F, t1068: F, t1070: F, t17194: F, t17197: F, t17198: F, t17202: F, t17209: F, t17301: F, t17303: F, t17306: F, t17372: F, t17374: F, t17377: F, t17379: F, t17425: F, t17427: F, t17561: F, t17563: F, t17568: F, t193: F, t336: F, t4696: F, t4700: F, t4701: F) -> (F, F) {
    let t18169 = t18050 + t18168;
    let t18173 = t1070 * t18169 * t193 * t336 + F::cast_from(2.0_f64) * t1068 * t17198 * t4700 - t1068 * t17202 * t4700 - F::cast_from(2.0_f64) * t4696 * t4700 * t4701 + t17194 + t17197 - t17209 - t17301 - t17303 - t17306 + t17372 + t17374 - t17377 + t17379 + t17425 + t17427 + t17561 - t17563 - t17568;
    (t18169, t18173)
}
