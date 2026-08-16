//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 636/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk636(t1550: f64, t9000: f64, t2295: f64, t5016: f64, t2034: f64, t6355: f64, t1679: f64, t2157: f64, t739: f64, t8997: f64, t132: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9062 = t1550 * t9000;
    let t9071 = t5016 * t2295;
    let t9073 = t6355 * t2034;
    let t9075 = t1679 * t2157;
    let t9079 = t739 * t8997;
    let t9081 = t577 * t132;
    (t9062, t9071, t9073, t9075, t9079, t9081)
}
