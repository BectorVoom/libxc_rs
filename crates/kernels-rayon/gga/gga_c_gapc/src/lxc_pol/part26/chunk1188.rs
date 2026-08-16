//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1188/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1188(t128: f64, t20569: f64, t647: f64, t1030: f64, t34106: f64, t33273: f64, t9053: f64, t11484: f64, t1688: f64, t20897: f64, t11313: f64, t26887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34711 = t20569 * t647 * t128;
    let t34712 = t1030 * t34106 * t34711;
    let t34714 = t1030 * t33273;
    let t34715 = t34714 * t9053;
    let t34718 = t11484 * t1688 * t20897;
    let t34720 = t26887 * t11313;
    (t34711, t34712, t34714, t34715, t34718, t34720)
}
