//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1319/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1319(t18377: f64, t1881: f64, t2233: f64, t27328: f64, t27723: f64, t637: f64, t8130: f64, t92165: f64, t92168: f64, t92170: f64, t92339: f64, t92344: f64, t92351: f64, t93817: f64, t97584: f64) -> f64 {
    let t99767 = t8130 * t27328 / 8.0_f64 - t2233 * t18377 * t637 / 16.0_f64 + t93817 - t92165 + t1881 * t27723 / 16.0_f64 + t97584 + t92168 + t92170 + t92339 + t92344 - t92351;
    t99767
}
