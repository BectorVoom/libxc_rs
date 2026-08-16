//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1938/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1938(t22779: f64, t26292: f64, t1339: f64, t54258: f64, t550: f64, t6936: f64, t22827: f64, t3788: f64, t3792: f64, t54068: f64, t12289: f64, t3791: f64, t54014: f64) -> (f64, f64, f64, f64) {
    let t91225 = t22779 * t26292;
    let t91229 = t6936 * t1339 * t54258 * t550;
    let t91233 = t22827 * t3788 * t54068 * t3792;
    let t91237 = t6936 * t12289 * t54014 * t3791;
    (t91225, t91229, t91233, t91237)
}
