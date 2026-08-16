//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1297/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1297(t20275: f64, t5483: f64, t1675: f64, t19380: f64, t5790: f64, t19345: f64, t18350: f64, t5492: f64, t19396: f64, t5791: f64, t18646: f64, t6073: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67451 = 16.0_f64 / 9.0_f64 * t5483 * t20275;
    let t67454 = 16.0_f64 / 9.0_f64 * t1675 * t5790 * t19380;
    let t67472 = t5790 * t19345;
    let t67474 = 160.0_f64 / 9.0_f64 * t18350 * t67472;
    let t67480 = 32.0_f64 / 9.0_f64 * t5492 * t20275;
    let t67491 = 32.0_f64 / 9.0_f64 * t19396 * t5791;
    let t67496 = t6073 * t18646;
    (t67451, t67454, t67472, t67474, t67480, t67491, t67496)
}
