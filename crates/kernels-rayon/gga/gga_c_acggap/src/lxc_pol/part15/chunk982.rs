//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 982/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk982(t30934: f64, t8450: f64, t30937: f64, t8597: f64, t8602: f64, t1165: f64, t4718: f64, t7351: f64, t7426: f64, t30543: f64, t8469: f64, t4521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34618 = t30934 * t8450;
    let t34620 = t30937 * t8597;
    let t34622 = t30937 * t8602;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34632 = t30543 * t8469;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    (t34618, t34620, t34622, t34626, t34632, t34636)
}
