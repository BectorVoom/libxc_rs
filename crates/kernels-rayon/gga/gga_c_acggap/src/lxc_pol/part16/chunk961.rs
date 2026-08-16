//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 961/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk961(t1181: f64, t23745: f64, t604: f64, t7493: f64, t31362: f64, t8775: f64, t30268: f64, t8956: f64, t1983: f64, t30262: f64, t7586: f64, t8406: f64) -> (f64, f64, f64, f64) {
    let t34099 = t7493 * t1181 * t604 * t23745;
    let t34100 = 0.21437009059034868486e-2_f64 * t34099;
    let t34101 = t31362 * t8775;
    let t34102 = 0.10718504529517434243e-2_f64 * t34101;
    let t34107 = t30268 * t8956;
    let t34127 = t30262 * t7586 * t1983 * t8406;
    (t34100, t34102, t34107, t34127)
}
