//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 608/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk608(t53: f64, t938: f64, t72: f64, t5591: f64, t2247: f64, t5578: f64, t2258: f64, t3052: f64, t11233: f64, t384: f64) -> (f64, f64, f64, f64) {
    let t25792 = t938 * t53;
    let t25793 = t72 * t25792;
    let t25794 = t5591 * t25793;
    let t25797 = t5578 * t2247;
    let t25798 = t2258 * t3052;
    let t25799 = t25797 * t25798;
    let t25802 = t11233 * t384;
    (t25794, t25798, t25799, t25802)
}
