//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2019/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2019(t177: f64, t4392: f64, t762: f64, t10605: f64, t162: f64, t4403: f64, t2626: f64, t4398: f64, t10439: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14322 = t4392 * t177;
    let t14324 = 0.11696447245269292414e1_f64 * t14322 * t762;
    let t14325 = t10605 * t162;
    let t14327 = 24.0_f64 * t14325 * t4403;
    let t14328 = t4398 * t2626;
    let t14329 = 0.11696447245269292414e1_f64 * t14328;
    let t14330 = t10439 * t162;
    (t14322, t14324, t14325, t14327, t14328, t14329, t14330)
}
