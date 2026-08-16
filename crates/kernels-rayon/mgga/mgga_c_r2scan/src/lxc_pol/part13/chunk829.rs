//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 829/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk829(t2551: f64, t7433: f64, t360: f64, t495: f64, t2294: f64, t2563: f64, t2133: f64, t259: f64, t547: f64, t6448: f64, t1593: f64, t2567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7449 = t7433 * t2551;
    let t7450 = t360 * t7449;
    let t7453 = t7433 * t495;
    let t7454 = t360 * t7453;
    let t7457 = t2294 * t2563;
    let t7459 = 0.23115257973478049502e0_f64 * t2133 * t7457;
    let t7460 = t547 * t259;
    let t7461 = t6448 * t7460;
    let t7462 = t2567 * t1593;
    (t7449, t7450, t7453, t7454, t7459, t7460, t7461, t7462)
}
