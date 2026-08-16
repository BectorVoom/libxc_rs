//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 487/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk487(t1036: f64, t245: f64, t2944: f64, t2952: f64, t3078: f64, t3081: f64, t3093: f64, t934: f64) -> f64 {
    let t3096 = -t3078 * t2944 / 8.0_f64 + t3081 * t934 / 2.0_f64 + t1036 * t2952 / 4.0_f64 + t245 * t3093 / 2.0_f64;
    t3096
}
