//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1343/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1343<F: Float>(t21136: F, t5791: F, t1791: F, t69152: F, t1792: F, t18666: F, t18673: F, t19342: F, t19352: F, t20264: F, t20282: F, t21146: F, t5794: F, t6073: F, t62019: F, t6304: F, t65189: F, t67326: F, t67510: F, t67512: F, t69147: F, t69186: F, t69281: F) -> F {
    let t71529 = t21136 * t5791;
    let t71535 = t1791 * t69152;
    let t71544 = t69281 * t1792 / F::cast_from(3.0_f64) + t21146 * t5794 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19352 * t6304 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6073 * t20282 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t71529 - F::cast_from(880.0_f64) / F::cast_from(27.0_f64) * t67510 - F::cast_from(352.0_f64) / F::cast_from(27.0_f64) * t67512 + F::cast_from(20.0_f64) * t18666 * t69147 - F::cast_from(20.0_f64) * t62019 * t71535 + F::cast_from(20.0_f64) * t67326 * t19342 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t65189 * t20264 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t69186 * t18673;
    t71544
}
