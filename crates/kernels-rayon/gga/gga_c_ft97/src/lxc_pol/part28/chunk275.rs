//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 275/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk275(t3052: f64, t378: f64, t3051: f64, t1639: f64, t1640: f64, t3042: f64, t3045: f64, t3048: f64, t35: f64, t374: f64, t1594: f64, t3037: f64) -> (f64, f64, f64, f64, f64) {
    let t3053 = t378 * t3052;
    let t3054 = t3051 * t3053;
    let t3056 = t1639 + t1640 / 9.0_f64 + t3042 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3045 + 2.0_f64 / 3.0_f64 * t3048 - 2.0_f64 / 3.0_f64 * t3054;
    let t3057 = t3056 * t35;
    let t3058 = t374 * t3057;
    let t3061 = t1594 * t3037;
    (t3054, t3056, t3057, t3058, t3061)
}
