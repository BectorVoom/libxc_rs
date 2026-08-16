//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 411/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk411(t2098: f64, t3: f64, t1401: f64, t2039: f64, t577: f64, t480: f64, t11: f64, t2: f64, t584: f64, t16: f64, t9: f64, t587: f64, t591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2099 = t3 * t2098;
    let t2105 = 0.45e1_f64 * t2098 * t577 + 0.135e2_f64 * t1401 * t2039;
    let t2130 = t480 * t480;
    let t2218 = 0.174e1_f64 * t11;
    let t2219 = t2 * t584;
    let t2220 = 0.696e1_f64 * t2219;
    let t2221 = t9 * t16;
    let t2222 = 0.1122e2_f64 * t2221;
    let t2223 = t587 * t591;
    (t2099, t2105, t2130, t2218, t2219, t2220, t2221, t2222, t2223)
}
