//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1088/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1088(t13334: f64, t38: f64, t1486: f64, t2251: f64, t2259: f64, t4217: f64, t607: f64, t1471: f64, t1487: f64, t1494: f64, t2252: f64, t2260: f64, t2263: f64, t2312: f64, t4196: f64, t4218: f64, t4238: f64, t608: f64, t641: f64, t85: f64) -> f64 {
    let t13335 = t38 * t13334;
    let t13340 = t2251 * t1486;
    let t13343 = t2259 * t1486;
    let t13346 = t607 * t4217;
    let t13363 = t13335 * t85 / 24.0_f64 - t1471 * t2312 / 12.0_f64 - t13340 * t85 / 12.0_f64 - t13343 * t85 / 12.0_f64 - t13346 * t85 / 6.0_f64 - t4196 * t641 / 6.0_f64 - t2252 * t1494 / 12.0_f64 - t2260 * t1494 / 12.0_f64 - t2263 * t1494 / 6.0_f64 - t608 * t4238 / 6.0_f64 + t4218 * t641 / 12.0_f64 + t1487 * t2312 / 24.0_f64;
    t13363
}
