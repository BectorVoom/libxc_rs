//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 642/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk642(t3290: f64, t924: f64, t2124: f64, t2604: f64, t3295: f64, t261: f64, t939: f64, t3299: f64, t943: f64, t3304: f64, t2608: f64, t3308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3586 = t3290 * t924;
    let t3588 = t2124 * t2604;
    let t3589 = t3295 * t3588;
    let t3591 = t261 * t939;
    let t3592 = t3299 * t3591;
    let t3594 = t261 * t943;
    let t3595 = t3304 * t3594;
    let t3597 = t3308 * t2608;
    (t3586, t3588, t3589, t3591, t3592, t3594, t3595, t3597)
}
