//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1298/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1298(t10310: f64, t7942: f64, t10293: f64, t10304: f64, t10315: f64, t10320: f64, t10325: f64, t2002: f64, t2022: f64, t20225: f64, t2028: f64, t2054: f64, t28183: f64, t28185: f64, t28187: f64, t3171: f64, t3177: f64, t3925: f64, t3938: f64, t572: f64, t6291: f64, t675: f64, t8296: f64) -> f64 {
    let t28189 = t7942 * t10310;
    let t28223 = t572 * t3177 * t10320 * t2002 / 27.0_f64 - 2.0_f64 / 81.0_f64 * t28183 + 2.0_f64 / 243.0_f64 * t28185 + 2.0_f64 / 27.0_f64 * t28187 + 44.0_f64 / 81.0_f64 * t28189 - t572 * t3177 * t10304 * t2002 / 9.0_f64 - 2.0_f64 / 81.0_f64 * t572 * t3171 * t2054 * t10325 * t675 - t572 * t3171 * t10315 * t2002 / 81.0_f64 - 5.0_f64 / 243.0_f64 * t572 * t8296 * t6291 * t3938 * t2028 + 2.0_f64 / 27.0_f64 * t572 * t3177 * t2022 * t10325 * t675 + 4.0_f64 / 9.0_f64 * t572 * t3177 * t10293 * t2028 + 20.0_f64 / 81.0_f64 * t572 * t8296 * t20225 * t3925 * t2028;
    t28223
}
