//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 760/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk760(t590: f64, t8937: f64, t5581: f64, t599: f64, t596: f64, t1043: f64, t1976: f64, t3081: f64, t8832: f64, t1736: f64, t3152: f64, t169: f64) -> (f64, f64, f64, f64, f64) {
    let t8938 = t590 * t8937;
    let t8940 = t5581 * t599;
    let t8941 = t596 * t8940;
    let t8943 = t1043 * t1976;
    let t8945 = t8832 * t3081;
    let t8947 = t3152 * t1736;
    let t8948 = t169 * t8947;
    (t8938, t8941, t8943, t8945, t8948)
}
