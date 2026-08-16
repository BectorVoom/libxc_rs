//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 964/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk964(t11210: f64, t1649: f64, t11208: f64, t3707: f64, t6: f64, t101: f64, t4050: f64, t4055: f64, t520: f64, t2933: f64, t3640: f64, t125: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11211 = t11210 * t1649;
    let t11212 = t11208 * t11211;
    let t11214 = t6 * t3707;
    let t11215 = t11214 * t101;
    let t11216 = t11215 * t4050;
    let t11217 = t520 * t4055;
    let t11218 = t11216 * t11217;
    let t11220 = t2933 * t3640;
    let t11222 = t125 * t505;
    (t11211, t11212, t11214, t11215, t11216, t11217, t11218, t11220, t11222)
}
