//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1649/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1649(t276: f64, t285: f64, t2881: f64, t918: f64, t273: f64, t2439: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11354 = 1.0_f64 / t276 / t285 / 4.0_f64;
    let t11355 = t2881 * t918;
    let t11356 = t11354 * t11355;
    let t11358 = 1.0_f64/pow_3_2(t273);
    let t11359 = t11358 * t11355;
    let t11366 = t2439 * t931;
    (t11354, t11355, t11356, t11358, t11359, t11366)
}
