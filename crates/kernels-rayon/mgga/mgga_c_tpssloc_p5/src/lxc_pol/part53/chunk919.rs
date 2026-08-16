//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 919/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk919(t218: f64, t33947: f64, t10110: f64, t1527: f64, t8733: f64, t259: f64, t31971: f64, t32014: f64, t33372: f64, t33410: f64, t33420: f64, t33423: f64, t33430: f64, t33935: f64, t33940: f64, t7087: f64, t7830: f64, t7842: f64, t855: f64) -> (f64, f64, f64) {
    let t33948 = t218 * t33947;
    let t33951 = t10110 * t8733 * t1527;
    let t33960 = -0.3289868133696452873e-1_f64 * t33372 - t31971 + 4.0_f64 * t855 * t33935 + 4.0_f64 * t7087 * t7830 + t33940 * t259 + t33948 * t259 - 6.0_f64 * t855 * t33951 - 0.3289868133696452873e-1_f64 * t33410 - t32014 - 0.6579736267392905746e-1_f64 * t33420 - 0.3289868133696452873e-1_f64 * t33423 + 0.3289868133696452873e-1_f64 * t33430 - 2.0_f64 * t7087 * t7842;
    (t33948, t33951, t33960)
}
