//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 652/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk652(t72: f64, t9064: f64, t1562: f64, t2131: f64, t2295: f64, t5016: f64, t2034: f64, t6355: f64, t1679: f64, t2157: f64, t2150: f64, t623: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9065 = t72 * t9064;
    let t9069 = t1562 * t2131;
    let t9071 = t5016 * t2295;
    let t9073 = t6355 * t2034;
    let t9075 = t1679 * t2157;
    let t9077 = t623 * t2150;
    (t9065, t9069, t9071, t9073, t9075, t9077)
}
