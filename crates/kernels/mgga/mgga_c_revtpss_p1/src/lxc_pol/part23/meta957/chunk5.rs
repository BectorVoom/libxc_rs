//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3206/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3206<F: Float>(t43888: F, t56236: F, t56343: F, t56345: F, t56360: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F) -> F {
    let t84174 = F::cast_from(0.9877777777777777778e-2_f64) * t68332 + F::cast_from(0.19755555555555555556e-1_f64) * t68334 + F::cast_from(0.59266666666666666668e-1_f64) * t68336 - t56343 + t56345 + F::cast_from(0.26670000000000000001e0_f64) * t81224 + F::cast_from(0.14816666666666666667e-1_f64) * t81228 - F::cast_from(0.5487654320987654321e-2_f64) * t81230 + F::cast_from(0.19755555555555555556e-1_f64) * t81232 - F::cast_from(0.29633333333333333334e-1_f64) * t81234 - F::cast_from(0.4938888888888888889e-2_f64) * t81236 + t56360 - F::cast_from(0.46096296296296296297e-1_f64) * t56236 - F::cast_from(0.14816666666666666667e-1_f64) * t68389 + F::cast_from(0.39511111111111111112e-1_f64) * t68399 + F::cast_from(0.4938888888888888889e-1_f64) * t81242 - F::cast_from(0.17780000000000000001e0_f64) * t81245 - F::cast_from(0.15365432098765432099e-1_f64) * t43888 - F::cast_from(0.59266666666666666668e-1_f64) * t68454 - F::cast_from(0.88900000000000000002e-1_f64) * t68456;
    t84174
}
