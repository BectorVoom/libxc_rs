//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 980/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk980(t1314: f64, t3897: f64, t455: f64, t3900: f64, t468: f64, t11407: f64, t1346: f64, t3943: f64, t3946: f64, t481: f64, t1311: f64, t3860: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11512 = 1.0_f64 / t3897 / t1314;
    let t11513 = t455 * t11512;
    let t11516 = 1.0_f64 / t3900 / t468;
    let t11520 = 0.28842592592592592592e-1_f64 * t11407;
    let t11536 = 1.0_f64 / t3943 / t1346;
    let t11539 = 1.0_f64 / t3946 / t481;
    let t11543 = t1311 * t3860;
    (t11513, t11516, t11520, t11536, t11539, t11543)
}
