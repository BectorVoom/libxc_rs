//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1912/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1912(t6420: f64, t6987: f64, t1825: f64, t26458: f64, t19743: f64, t550: f64, t6976: f64, t1992: f64, t1336: f64, t22718: f64, t22726: f64, t26437: f64, t27096: f64, t28156: f64, t28161: f64, t28165: f64, t28169: f64, t28171: f64, t5234: f64, t544: f64, t7745: f64) -> (f64, f64, f64, f64, f64) {
    let t28174 = t6987 * t6420;
    let t28178 = t26458 * t1825;
    let t28181 = t19743 * t550;
    let t28182 = t6976 * t28181;
    let t28183 = t1992 * t28182;
    let t28185 = t544 * t28156 - t27096 - 0.82246703342411321824e-2_f64 * t26437 + 0.82246703342411321825e-2_f64 * t28161 + t22718 + t22726 - 0.16449340668482264365e-1_f64 * t28165 - 0.82246703342411321825e-2_f64 * t28169 + 2.0_f64 * t1336 * t28171 - t1336 * t28174 - 2.0_f64 * t5234 * t7745 - 2.0_f64 * t1336 * t28178 - 0.82246703342411321825e-2_f64 * t28183;
    (t28174, t28178, t28181, t28182, t28185)
}
