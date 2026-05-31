//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1454/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1454<F: Float>(t324: F, t41525: F, t41538: F, t300: F, t41306: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t41330: F, t41332: F, t41334: F, t41336: F) -> (F, F, F) {
    let t41540 = (t41525 + t41538) * t324;
    let t41542 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t41540;
    let t41549 = F::cast_from(0.18467901234567901234e0_f64) * t41306;
    let t41554 = F::cast_from(0.14246666666666666667e0_f64) * t41308 + F::cast_from(0.4274e0_f64) * t41312 - F::cast_from(0.6411e0_f64) * t41316 + F::cast_from(0.10685e0_f64) * t41320 + F::cast_from(0.42739999999999999999e0_f64) * t41323 - F::cast_from(0.35616666666666666666e-1_f64) * t41327 + t41549 - F::cast_from(0.47488888888888888888e-1_f64) * t41330 - F::cast_from(0.31659259259259259258e-1_f64) * t41332 + F::cast_from(0.23744444444444444444e-1_f64) * t41334 + F::cast_from(0.26382716049382716049e-1_f64) * t41336;
    (t41540, t41542, t41554)
}
