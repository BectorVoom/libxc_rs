//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 882/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk882(t10211: f64, t10212: f64, t770: f64, t791: f64, t2431: f64, t3197: f64, t6791: f64, t831: f64, t1062: f64, t2200: f64, t2212: f64, t3254: f64) -> (f64, f64, f64, f64) {
    let t10213 = t10211 * t10212;
    let t10215 = t791 * t770;
    let t10216 = t3197 * t2431;
    let t10217 = t10215 * t10216;
    let t10219 = t6791 * t831;
    let t10220 = t1062 * t10219;
    let t10222 = t2200 * t2212;
    let t10223 = t3254 * t10222;
    (t10213, t10217, t10220, t10223)
}
