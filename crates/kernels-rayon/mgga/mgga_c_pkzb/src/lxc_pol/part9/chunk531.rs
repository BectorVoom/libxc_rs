//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 531/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk531(t2198: f64, t852: f64, t2197: f64, t336: f64, t339: f64) -> (f64, f64, f64) {
    let t2199 = t2198 * t852;
    let t2201 = 2.0_f64 * t2197 * t2199;
    let t2203 = 1.0_f64 / t339 / t336;
    (t2199, t2201, t2203)
}
