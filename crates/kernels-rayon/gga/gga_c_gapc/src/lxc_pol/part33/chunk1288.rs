//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1288/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1288(t11679: f64, t24092: f64, t6182: f64, t9497: f64, t10346: f64, t134: f64, t6939: f64, t11210: f64, t11657: f64, t7557: f64, t11662: f64, t2200: f64, t6857: f64) -> (f64, f64, f64, f64, f64) {
    let t35890 = t11679 * t24092;
    let t35894 = t6182 * t9497;
    let t35895 = t10346 * t6939 * t134 * t35894;
    let t35898 = t11657 * t11210 * t7557;
    let t35901 = t11662 * t2200 * t6857;
    (t35890, t35894, t35895, t35898, t35901)
}
