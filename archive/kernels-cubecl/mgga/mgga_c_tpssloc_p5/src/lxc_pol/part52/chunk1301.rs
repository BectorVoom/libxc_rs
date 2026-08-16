//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1301/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1301<F: Float>(t118429: F, t118964: F, t113111: F, t113117: F, t118399: F, t118948: F, t1484: F, t1530: F, t16596: F, t1877: F, t193: F, t202: F, t2522: F, t25365: F, t25374: F, t30753: F, t30757: F, t32886: F, t4119: F, t4255: F, t4303: F, t4314: F, t776: F, t8366: F, t868: F, t870: F) -> (F, F) {
    let t118965 = t118429 + t118964;
    let t119639 = t118948 * t193 * t202 * t870 - t113111 * t1530 * t1877 + F::cast_from(2.0_f64) * t113117 * t1877 * t25374 - t118399 * t1877 * t868 + F::cast_from(3.0_f64) * t1484 * t2522 * t30753 - F::cast_from(3.0_f64) * t16596 * t2522 * t30757 - t1877 * t30757 * t4303 - F::cast_from(3.0_f64) * t2522 * t25365 * t30757 + F::cast_from(3.0_f64) * t2522 * t32886 * t776 + F::cast_from(3.0_f64) * t2522 * t4119 * t8366 + F::cast_from(6.0_f64) * t4255 * t4314 * t8366;
    (t118965, t119639)
}
