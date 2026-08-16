//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 400/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk400(t1417: f64, t1947: f64, t1044: f64, t6: f64, t125: f64, t611: f64, t1418: f64, t147: f64) -> (f64, f64, f64, f64) {
    let t1948 = t1417 * t1947;
    let t1951 = t6 * t1044;
    let t1952 = t1951 * t125;
    let t1953 = t611 * t1952;
    let t1954 = t1418 * t147;
    (t1948, t1952, t1953, t1954)
}
