//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1299/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1299<F: Float>(t10295: F, t1890: F, t10290: F, t10299: F, t7942: F, t10288: F, t20278: F, t2028: F, t20280: F, t20282: F, t20355: F, t24131: F, t24133: F, t24135: F, t24137: F, t3171: F, t572: F) -> F {
    let t28228 = t1890 * t10295;
    let t28230 = t1890 * t10290;
    let t28232 = t7942 * t10299;
    let t28242 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t572 * t3171 * t10288 * t2028 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t28228 + F::cast_from(10.0_f64) / F::cast_from(729.0_f64) * t28230 - F::cast_from(44.0_f64) / F::cast_from(243.0_f64) * t28232 + F::cast_from(4.0_f64) / F::cast_from(243.0_f64) * t20355 + F::cast_from(28.0_f64) / F::cast_from(729.0_f64) * t20278 - F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t20280 - F::cast_from(4.0_f64) / F::cast_from(729.0_f64) * t20282 + F::cast_from(16.0_f64) / F::cast_from(243.0_f64) * t24131 - F::cast_from(16.0_f64) / F::cast_from(729.0_f64) * t24133 + F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t24135 + F::cast_from(10.0_f64) / F::cast_from(729.0_f64) * t24137;
    t28242
}
