//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1048/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1048(t10666: f64, t113: f64, t97: f64, t1561: f64, t3261: f64, t105: f64, t1550: f64, t122: f64, t874: f64, t3438: f64, t10978: f64, t10979: f64, t2317: f64) -> (f64, f64, f64, f64, f64) {
    let t37282 = t97 * t10666 * t113;
    let t37327 = t97 * t3261 * t1561;
    let t37346 = t97 * t105 * t1550;
    let t37355 = t874 * t122;
    let t37356 = t3438 * t37355;
    let t37358 = t10978 * t10979 * t2317 * t37356;
    (t37282, t37327, t37346, t37355, t37358)
}
