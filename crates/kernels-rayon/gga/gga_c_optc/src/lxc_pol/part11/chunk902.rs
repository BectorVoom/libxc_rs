//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 902/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk902(t16988: f64, t2672: f64, t935: f64, t313: f64, t16644: f64, t2722: f64, t16225: f64, t7865: f64, t894: f64, t16636: f64, t3608: f64, t7857: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16990 = t16988 * t2672 * t935;
    let t16991 = t313 * t16990;
    let t16994 = t2722 * t16644;
    let t16997 = t7865 * t16225;
    let t16998 = t894 * t16997;
    let t17001 = t3608 * t16636;
    let t17004 = t7857 * t16225;
    (t16990, t16991, t16994, t16997, t16998, t17001, t17004)
}
