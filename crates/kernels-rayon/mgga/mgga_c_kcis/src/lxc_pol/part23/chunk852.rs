//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 852/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk852(t16120: f64, t3902: f64, t11571: f64, t1907: f64, t3856: f64, t5574: f64, t13948: f64, t5570: f64, t1903: f64, t2331: f64, t11491: f64, t1897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16122 = 0.16081824322151104822e2_f64 * t16120 * t3902;
    let t16124 = 1.0_f64 * t11571 * t1907;
    let t16126 = 2.0_f64 * t3856 * t5574;
    let t16127 = t13948 * t5570;
    let t16129 = t2331 * t1903;
    let t16131 = t11491 * t1897;
    (t16122, t16124, t16126, t16127, t16129, t16131)
}
