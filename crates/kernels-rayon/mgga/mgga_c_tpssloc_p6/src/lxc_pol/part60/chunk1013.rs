//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1013/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1013(t1914: f64, t5527: f64, t5660: f64, t5664: f64, t101840: f64, t115009: f64, t121782: f64, t126180: f64, t126198: f64, t126530: f64, t1877: f64, t22960: f64, t24191: f64, t2522: f64, t25373: f64, t26563: f64, t26744: f64, t26756: f64, t28241: f64, t28249: f64, t28256: f64, t32899: f64, t33476: f64, t33477: f64, t33483: f64, t33484: f64, t4314: f64, t7114: f64, t7545: f64, t8566: f64, t86716: f64, t86721: f64, t92319: f64, t98064: f64) -> (f64, f64, f64, f64) {
    let t128097 = t1914 * t5527;
    let t128101 = t1914 * t5660;
    let t128110 = t1914 * t5664;
    let t128134 = 3.0_f64 * t4314 * t8566 * t28241 - 3.0_f64 * t26563 * t22960 * t128097 + t26756 * t25373 * t128101 + 2.0_f64 * t26756 * t126198 - t1877 * t26744 * t32899 - 3.0_f64 * t115009 * t28249 - 3.0_f64 * t26756 * t86716 * t128110 - 3.0_f64 * t92319 * t33477 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t28256 - t1877 * t121782 * t7545 + 2.0_f64 * t101840 * t33484 - t1877 * t7114 * t126180 / 2.0_f64 - 3.0_f64 * t24191 * t86721 * t33476 + 2.0_f64 * t26756 * t98064 * t33483 - t1877 * t7114 * t126530;
    (t128097, t128101, t128110, t128134)
}
