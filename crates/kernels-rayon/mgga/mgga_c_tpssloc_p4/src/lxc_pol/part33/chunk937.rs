//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 937/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk937(t1380: f64, t20568: f64, t1825: f64, t19660: f64, t5348: f64, t6420: f64, t20473: f64, t5335: f64, t20554: f64, t6415: f64, t19657: f64, t16428: f64, t6388: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20630 = t1380 * t20568;
    let t20632 = t19660 * t1825;
    let t20635 = t5348 * t6420;
    let t20638 = t5335 * t20473;
    let t20643 = t1380 * t20554;
    let t20645 = t5348 * t6415;
    let t20648 = t19657 * t1825;
    let t20651 = t16428 * t6388;
    (t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651)
}
