//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1069/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1069(t11756: f64, t11762: f64, t11773: f64, t11776: f64, t12193: f64, t12194: f64, t12195: f64, t12196: f64, t12197: f64, t12198: f64, t12199: f64, t12200: f64, t12203: f64, t12204: f64, t12205: f64, t12208: f64, t12209: f64, t12210: f64, t12211: f64) -> f64 {
    let t12633 = t12193 + t12194 - t12195 + t12196 + t12197 - t12198 - t12199 + t12200 - 0.25297741735382421301e-7_f64 * t11756 + 0.12228868272569444445e-4_f64 * t11762 - t12203 - t12204 + t12205 + 0.12650553385416666667e-5_f64 * t11773 + 0.12650553385416666667e-5_f64 * t11776 + t12208 + t12209 + t12210 + t12211;
    t12633
}
