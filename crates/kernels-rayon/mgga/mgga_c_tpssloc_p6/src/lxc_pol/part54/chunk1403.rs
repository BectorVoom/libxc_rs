//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1403/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1403(t114992: f64, t118413: f64, t118953: f64, t121774: f64, t121779: f64, t1530: f64, t1877: f64, t1914: f64, t193: f64, t202: f64, t24344: f64, t2522: f64, t25353: f64, t25365: f64, t25374: f64, t26744: f64, t26756: f64, t31434: f64, t31448: f64, t33466: f64, t33483: f64, t4119: f64, t4255: f64, t4314: f64, t6665: f64, t7114: f64, t776: f64, t82312: f64, t84800: f64, t8566: f64, t870: f64, t92276: f64, t93000: f64) -> f64 {
    let t121949 = 2.0_f64 * t1877 * t93000 * t31448 - 6.0_f64 * t26756 * t82312 * t25374 + 3.0_f64 * t2522 * t8566 * t4119 - 3.0_f64 * t2522 * t31434 * t25365 + t193 * t202 * t121774 * t870 - t1877 * t92276 * t1914 + 3.0_f64 * t2522 * t33466 * t776 + 6.0_f64 * t4314 * t8566 * t4255 - t1877 * t7114 * t25353 - t1877 * t114992 * t1530 - t1877 * t26744 * t6665 + 2.0_f64 * t1877 * t24344 * t118953 + 2.0_f64 * t1877 * t24344 * t118413 + 2.0_f64 * t1877 * t24344 * t121779 + 2.0_f64 * t1877 * t84800 * t33483;
    t121949
}
