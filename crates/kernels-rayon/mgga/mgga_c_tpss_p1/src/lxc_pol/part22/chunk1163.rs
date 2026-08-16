//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1163/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1163(t124: f64, t12810: f64, t762: f64, t10111: f64, t4415: f64, t4416: f64, t10117: f64, t4425: f64, t4466: f64, t3261: f64, t3273: f64, t4471: f64) -> (f64, f64, f64, f64, f64) {
    let t12995 = t124 * t12810;
    let t12996 = t762 * t12995;
    let t13000 = t4415 * t4416 * t10111;
    let t13004 = 7.0_f64 / 576.0_f64 * t10117 * t4425;
    let t13006 = 7.0_f64 / 2304.0_f64 * t10117 * t4466;
    let t13009 = t3273 * t4471 * t3261;
    (t12996, t13000, t13004, t13006, t13009)
}
