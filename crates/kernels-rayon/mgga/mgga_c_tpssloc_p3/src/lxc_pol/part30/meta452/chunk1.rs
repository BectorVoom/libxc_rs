//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1718/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1718(t22927: f64, t6897: f64, t225: f64, t3886: f64, t6883: f64, t6903: f64, t1914: f64, t193: f64, t201: f64) -> (f64, f64, f64, f64) {
    let t22928 = t6897 * t22927;
    let t22933 = t225 * t3886;
    let t22940 = t6883 * t6903;
    let t22959 = t193 * t201 * t1914;
    (t22928, t22933, t22940, t22959)
}
