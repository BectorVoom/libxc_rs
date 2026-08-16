//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 399/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk399(t1004: f64, t109: f64, t108: f64, t500: f64, t1968: f64, t1978: f64) -> (f64, f64, f64) {
    let t3134 = t1004 * t109;
    let t3142 = t500 * t108;
    let t3350 = t1978 * t1968;
    (t3134, t3142, t3350)
}
