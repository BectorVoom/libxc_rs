//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 900/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk900(t1228: f64, t3139: f64, t1213: f64, t2999: f64, t89: f64, t1186: f64, t3704: f64, t10696: f64, t1240: f64, t10478: f64, t2770: f64, t4246: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55274 = t3139 * t1228;
    let t55558 = t89 * t2999 * t1213;
    let t55562 = t89 * t3704 * t1186;
    let t55768 = t1240 * t10696;
    let t55937 = t10478 * t1240;
    let t56098 = t2770 * t4246;
    (t55274, t55558, t55562, t55768, t55937, t56098)
}
