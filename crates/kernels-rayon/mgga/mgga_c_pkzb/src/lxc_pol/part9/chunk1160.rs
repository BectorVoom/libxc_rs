//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1160/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1160(t1769: f64, t6992: f64, t2642: f64, t5384: f64, t16193: f64, t16230: f64, t16273: f64, t16275: f64, t16280: f64, t16283: f64, t16287: f64, t16290: f64, t19621: f64, t19624: f64, t19626: f64, t19628: f64, t19686: f64, t19688: f64, t19690: f64, t19691: f64) -> (f64, f64, f64) {
    let t20272 = t1769 * t6992;
    let t20274 = t5384 * t2642;
    let t20275 = 0.17006693853500995666e-1_f64 * t20274;
    let t20317 = -t16193 - t16230 - t16273 + t16275 - t19621 + t19624 + t19626 + t19628 - t16280 + t19686 + t16283 + t16287 - t16290 + t19688 - t19690 + t19691;
    (t20272, t20275, t20317)
}
