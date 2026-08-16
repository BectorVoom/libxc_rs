//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1014/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1014(t11337: f64, t240: f64, t3252: f64, t276: f64, t285: f64, t273: f64, t2439: f64, t931: f64, t2922: f64, t913: f64, t275: f64, t290: f64, t2925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11338 = 0.36514074074074074075e0_f64 * t11337;
    let t11341 = t240 * t3252;
    let t11354 = 1.0_f64 / t276 / t285 / 4.0_f64;
    let t11358 = 1.0_f64/pow_3_2(t273);
    let t11366 = t2439 * t931;
    let t11384 = 1.0_f64 / t2922 / t913;
    let t11385 = t275 * t11384;
    let t11387 = 1.0_f64 / t2925 / t290;
    (t11338, t11341, t11354, t11358, t11366, t11385, t11387)
}
