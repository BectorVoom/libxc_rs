//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 645/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk645<F: Float>(t185: F, t5357: F, t5081: F, t5042: F, t5069: F, t5072: F, t5075: F, t5078: F, t5083: F, t5085: F, t5087: F, t5091: F, t5094: F, t203: F, t184: F, t221: F) -> (F, F, F, F, F) {
    let t5359 = 16.0 / 405.0 * t185 * t5357;
    let t5360 = 0.58774074074074074074e-2 * t5081;
    let t5371 = t5360 + 0.25188888888888888889e-2 * t5083 - 0.12594444444444444445e-2 * t5087 + 0.37783333333333333335e-2 * t5042 - 0.18891666666666666667e-2 * t5085 + 0.20990740740740740742e-2 * t5091 - 0.75566666666666666669e-2 * t5069 + 0.37783333333333333335e-2 * t5072 + 0.11335e-1 * t5075 - 0.11335e-1 * t5078 + 0.18891666666666666667e-2 * t5094;
    let t5372 = t203 * t5371;
    let t5373 = t5372 * t184;
    let t5375 = 2.0 / 15.0 * t5373 * t221;
    (t5359, t5371, t5372, t5373, t5375)
}
