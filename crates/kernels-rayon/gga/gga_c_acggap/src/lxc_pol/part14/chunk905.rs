//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 905/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk905(t7310: f64, t7389: f64, t7753: f64, t7799: f64, t435: f64, t7322: f64, t7323: f64, t1072: f64, t372: f64, t721: f64, t2019: f64, t2059: f64) -> (f64, f64, f64, f64, f64) {
    let t31126 = t7310 * t7389;
    let t31128 = t7799 * t7753;
    let t31137 = t7322 * t7323 * t435;
    let t31140 = t31137 * t1072 * t372 * t721;
    let t31142 = t2019 * t2059;
    (t31126, t31128, t31137, t31140, t31142)
}
