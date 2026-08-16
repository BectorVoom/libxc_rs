//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2624/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2624(t3809: f64, t53945: f64, t120: f64, t16205: f64, t12283: f64, t16227: f64, t1351: f64, t5286: f64, t12189: f64, t5227: f64, t16232: f64, t3777: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53946 = t53945 * t3809;
    let t53958 = t120 * t16205;
    let t53965 = t12283 * t16227;
    let t53973 = t5286 * t1351;
    let t53984 = t12189 * t5227;
    let t53990 = t3777 * t16232;
    (t53946, t53958, t53965, t53973, t53984, t53990)
}
