//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 838/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk838(t43: f64, t50: f64, t16225: f64, t16231: f64, t3331: f64, t4565: f64, t607: f64, t6533: f64, t16236: f64, t16241: f64, t3339: f64, t4573: f64, t611: f64, t6547: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t16277 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t6533 * t16225 - 2.0_f64 / 3.0_f64 * t3331 * t4565 + 2.0_f64 / 3.0_f64 * t607 * t16231);
    let t16285 = piecewise3(t51, 0.0_f64, 8.0_f64 / 27.0_f64 * t6547 * t16236 - 2.0_f64 / 3.0_f64 * t3339 * t4573 + 2.0_f64 / 3.0_f64 * t611 * t16241);
    (t16277, t16285)
}
