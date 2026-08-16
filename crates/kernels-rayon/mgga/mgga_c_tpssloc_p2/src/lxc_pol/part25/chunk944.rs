//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 944/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk944(t12155: f64, t12156: f64, t1307: f64, t1365: f64, t3719: f64, t12012: f64, t1347: f64, t12147: f64, t1345: f64, t1348: f64, t3839: f64, t3844: f64, t3847: f64, t5278: f64, t546: f64, t548: f64) -> f64 {
    let t12157 = t12155 * t12156;
    let t12160 = t1365 * t1307;
    let t12161 = t12160 * t3719;
    let t12164 = t1347 * t12012;
    let t12167 = -t12147 * t548 + 60.0_f64 * t12157 * t546 - 36.0_f64 * t12161 * t5278 + 3.0_f64 * t12164 * t546 - 36.0_f64 * t1345 * t3844 + 9.0_f64 * t1345 * t3847 + 9.0_f64 * t1348 * t3839;
    t12167
}
