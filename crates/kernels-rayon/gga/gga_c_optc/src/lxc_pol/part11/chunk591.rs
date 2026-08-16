//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 591/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk591(t1426: f64, t2301: f64, t350: f64, t4009: f64, t4831: f64, t4835: f64, t4846: f64, t974: f64, t275: f64, t176: f64, t1366: f64, sigma0: f64) -> (f64, f64, f64) {
    let t4848 = -2.0_f64 * t4009 * t1426 + 2.0_f64 * t2301 * t4835 + t4831 * t350 - t974 * t4846;
    let t4849 = t4848 * t275;
    let t4851 = t176 * t4849 * sigma0;
    let t4854 = t1366 * t1366;
    (t4848, t4851, t4854)
}
