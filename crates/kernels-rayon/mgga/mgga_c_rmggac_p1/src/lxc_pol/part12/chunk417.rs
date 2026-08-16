//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 417/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk417(t4084: f64, t946: f64, t249: f64, t973: f64, t945: f64, t1004: f64, t1243: f64, t1090: f64, t1101: f64, t378: f64, t483: f64, t7: f64) -> (f64, f64, f64, f64, f64) {
    let t4085 = t4084 * t946;
    let t4087 = t249 * t973;
    let t4089 = 0.16265371950452609763e-1_f64 * t945 * t4087;
    let t4090 = t1004 * t1243;
    let t4101 = 6.0_f64 * t1090 * t378 * t1101;
    let t4103 = t7 * t483;
    (t4085, t4089, t4090, t4101, t4103)
}
