//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1232/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1232(t1307: f64, t16195: f64, t3719: f64, t5279: f64, t1347: f64, t16018: f64, t1345: f64, t1348: f64, t16176: f64, t16186: f64, t16192: f64, t1819: f64, t1821: f64, t3839: f64, t3844: f64, t3847: f64, t5272: f64, t5278: f64, t5280: f64, t5283: f64, t546: f64, t548: f64) -> f64 {
    let t16196 = t16195 * t1307;
    let t16199 = t5279 * t3719;
    let t16202 = t1347 * t16018;
    let t16205 = 6.0_f64 * t1345 * t5283 + 6.0_f64 * t1348 * t5272 - t16176 * t548 - 24.0_f64 * t16186 * t5280 + 60.0_f64 * t16192 * t5278 - 24.0_f64 * t16196 * t5278 - 12.0_f64 * t16199 * t5278 + 3.0_f64 * t16202 * t546 - 12.0_f64 * t1819 * t3844 + 3.0_f64 * t1819 * t3847 + 3.0_f64 * t1821 * t3839;
    t16205
}
