//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 949/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk949(t19977: f64, t422: f64, t528: f64, t61819: f64, t929: f64, t20612: f64, t8959: f64, t20603: f64, t1554: f64, t20596: f64, t39942: f64, t20859: f64, t8392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76914 = t422 * t19977 * t528;
    let t76918 = t61819 * t929;
    let t76926 = t8959 * t20612;
    let t76928 = t8959 * t20603;
    let t76945 = t1554 * t19977;
    let t76982 = 0.22136921132726965153e-3_f64 * t39942 * t20596;
    let t77196 = t8392 * t20859;
    (t76914, t76918, t76926, t76928, t76945, t76982, t77196)
}
