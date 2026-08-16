//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2264/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2264(t28264: f64, t6547: f64, t225: f64, t28282: f64, t17022: f64, t1880: f64, t214: f64, t258: f64, t28272: f64, t6562: f64, t794: f64, t25224: f64, t25341: f64, t6552: f64) -> (f64, f64, f64, f64, f64) {
    let t99003 = t6547 * t28264;
    let t99010 = t28282 * t225;
    let t99019 = t1880 * t214 * t17022 * t225 * t258;
    let t99022 = t6562 * t794 * t28272;
    let t99033 = t6552 * t25224 * t25341;
    (t99003, t99010, t99019, t99022, t99033)
}
