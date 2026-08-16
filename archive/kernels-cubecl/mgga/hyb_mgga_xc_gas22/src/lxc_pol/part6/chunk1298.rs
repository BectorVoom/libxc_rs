//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1298/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1298<F: Float>(t10310: F, t7942: F, t10293: F, t10304: F, t10315: F, t10320: F, t10325: F, t2002: F, t2022: F, t20225: F, t2028: F, t2054: F, t28183: F, t28185: F, t28187: F, t3171: F, t3177: F, t3925: F, t3938: F, t572: F, t6291: F, t675: F, t8296: F) -> F {
    let t28189 = t7942 * t10310;
    let t28223 = t572 * t3177 * t10320 * t2002 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t28183 + F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t28185 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t28187 + F::cast_from(44.0_f64) / F::cast_from(81.0_f64) * t28189 - t572 * t3177 * t10304 * t2002 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t572 * t3171 * t2054 * t10325 * t675 - t572 * t3171 * t10315 * t2002 / F::cast_from(81.0_f64) - F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t572 * t8296 * t6291 * t3938 * t2028 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t572 * t3177 * t2022 * t10325 * t675 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t572 * t3177 * t10293 * t2028 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t572 * t8296 * t20225 * t3925 * t2028;
    t28223
}
