//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1265/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1265(t43: f64, t22028: f64, t22035: f64, t8414: f64, t553: f64, t6554: f64, t3145: f64, t9: f64, t2849: f64, t1122: f64, t1897: f64, t3119: f64, t3116: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t26112 = piecewise3(t44, 0.0_f64, t22028);
    let t26115 = t8414 * t22035;
    let t26122 = t6554 * t553;
    let t26133 = t9 * t3145;
    let t26134 = t26133 * t2849;
    let t26135 = t1897 * t1122;
    let t26136 = t26135 * t3119;
    let t26138 = t3116 * t26134 * t26136;
    (t26112, t26115, t26122, t26138)
}
