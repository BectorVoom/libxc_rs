//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 435/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk435(t2316: f64, t772: f64, t268: f64, t78: f64, t760: f64, t9: f64, t2152: f64, t22: f64, t2250: f64, t2254: f64, t2299: f64, t768: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2317 = t772 * t2316;
    let t2320 = t78 * t268;
    let t2324 = t9 * t760;
    let t2331 = t22 * t2152;
    let t2338 = t2250 * t2254;
    let t2342 = t768 * t2299;
    (t2317, t2320, t2324, t2331, t2338, t2342)
}
