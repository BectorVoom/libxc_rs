//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 540/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk540(t143: f64, t655: f64, t130: f64, t675: f64, t676: f64) -> (f64, f64, f64, f64, f64) {
    let t2286 = t655 * t143;
    let t2287 = 1.0_f64 / t2286;
    let t2288 = t130 * t2287;
    let t2289 = t675 * t675;
    let t2290 = t2289 * t676;
    let t2292 = 2.0_f64 * t2288 * t2290;
    (t2287, t2288, t2289, t2290, t2292)
}
