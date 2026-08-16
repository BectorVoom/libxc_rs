//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 423/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk423(t2210: f64, t2212: f64, t268: f64, t492: f64, t798: f64, t1482: f64, t827: f64, t462: f64, t760: f64, t513: f64, t786: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2213 = t2210 * t2212;
    let t2216 = t492 * t268;
    let t2217 = t2216 * t798;
    let t2220 = t1482 * t268;
    let t2221 = t2220 * t827;
    let t2224 = t462 * t760;
    let t2225 = t2224 * t798;
    let t2228 = t513 * t760;
    let t2229 = t2228 * t827;
    let t2232 = t786 * t875;
    (t2213, t2216, t2217, t2221, t2224, t2225, t2229, t2232)
}
