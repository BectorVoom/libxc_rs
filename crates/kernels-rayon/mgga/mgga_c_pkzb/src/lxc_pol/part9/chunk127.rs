//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 127/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk127(t336: f64, t339: f64, t342: f64, t346: f64) -> (f64, f64, f64) {
    let t361 = 0.705945e1_f64 * t339 + 0.1549425e1_f64 * t336 + 0.420775e0_f64 * t342 + 0.1562925e0_f64 * t346;
    let t364 = 1.0_f64 + 0.32163958997385070134e2_f64 / t361;
    let t365 = f64::ln(t364);
    (t361, t364, t365)
}
