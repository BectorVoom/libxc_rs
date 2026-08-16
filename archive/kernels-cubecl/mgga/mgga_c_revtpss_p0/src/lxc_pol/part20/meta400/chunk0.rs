//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1483/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1483<F: Float>(t378: F, t42051: F, t11198: F, t340: F, t338: F, t3059: F, t11119: F, t384: F, t225: F, t3270: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F) -> (F, F, F, F, F, F, F) {
    let t42052 = t42051 * t378;
    let t42058 = F::cast_from(1.0_f64) / t11198 / t340;
    let t42059 = t338 * t42058;
    let t42060 = t42059 * t378;
    let t42061 = t3059 * t3059;
    let t42066 = F::cast_from(1.0_f64) / t11119 / t384;
    let t42067 = t225 * t42066;
    let t42068 = t3270 * t3270;
    let t42078 = F::cast_from(0.15365432098765432099e0_f64) * t41306;
    let t42083 = F::cast_from(0.11853333333333333334e0_f64) * t41308 + F::cast_from(0.35560000000000000001e0_f64) * t41312 - F::cast_from(0.53340000000000000002e0_f64) * t41316 + F::cast_from(0.88900000000000000002e-1_f64) * t41320 + F::cast_from(0.35560000000000000001e0_f64) * t41323 - F::cast_from(0.29633333333333333334e-1_f64) * t41327 + t42078 - F::cast_from(0.39511111111111111112e-1_f64) * t41330 - F::cast_from(0.26340740740740740742e-1_f64) * t41332 + F::cast_from(0.19755555555555555556e-1_f64) * t41334 + F::cast_from(0.21950617283950617284e-1_f64) * t41336;
    (t42052, t42059, t42060, t42061, t42067, t42068, t42083)
}
