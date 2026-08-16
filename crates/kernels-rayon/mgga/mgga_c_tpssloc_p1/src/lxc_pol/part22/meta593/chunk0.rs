//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2109/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2109(t47093: f64, t4159: f64, t9541: f64, t1516: f64, t41052: f64, t4166: f64, t9600: f64, t849: f64, t13176: f64, t2696: f64, t1509: f64, t9975: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47094 = 119.0_f64 / 4608.0_f64 * t47093;
    let t47230 = t9541 * t4159;
    let t47231 = 35.0_f64 / 72.0_f64 * t47230;
    let t47269 = t41052 * t1516;
    let t47270 = 119.0_f64 / 1152.0_f64 * t47269;
    let t47275 = t4166 * t9600;
    let t47276 = t47275 * t849;
    let t47277 = 119.0_f64 / 1152.0_f64 * t47276;
    let t47278 = t13176 * t2696;
    let t47285 = t1509 * t9975;
    (t47094, t47231, t47270, t47275, t47277, t47278, t47285)
}
