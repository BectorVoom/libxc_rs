//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 894/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk894(t1584: f64, t2620: f64, t506: f64, t529: f64, t8048: f64, t1567: f64, t978: f64, t255: f64, t571: f64, t2086: f64, t980: f64, t538: f64, t7195: f64) -> (f64, f64, f64, f64, f64) {
    let t8189 = 0.23115257973478049502e0_f64 * t1584 * t2620;
    let t8191 = t529 * t506 * t8048;
    let t8196 = t1567 * t978;
    let t8198 = t571 * t8196 * t255;
    let t8201 = t980 * t2086;
    let t8204 = t529 * t538 * t7195;
    (t8189, t8191, t8198, t8201, t8204)
}
