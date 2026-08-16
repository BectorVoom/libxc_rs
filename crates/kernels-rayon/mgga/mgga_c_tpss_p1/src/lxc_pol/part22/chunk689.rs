//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 689/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk689(t189: f64, t3297: f64, t489: f64, t2281: f64, t2285: f64, t2292: f64, t2302: f64, t2310: f64, t3189: f64, t3199: f64, t3201: f64, t3209: f64, t3281: f64) -> (f64, f64, f64) {
    let t3298 = t3297 * t189;
    let t3299 = t489 * t3298;
    let t3300 = t2302 + t2310 - t2292 - t2281 - t2285 + t3281 + t3299 + t3199 - t3201 - t3209 + t3189;
    (t3298, t3299, t3300)
}
