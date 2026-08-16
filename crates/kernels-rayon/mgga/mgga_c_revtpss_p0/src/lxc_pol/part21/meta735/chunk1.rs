//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2585/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2585(t10174: f64, t2453: f64, t9676: f64, t123: f64, t2434: f64, t3915: f64, t4131: f64, t10175: f64, t9686: f64, t1420: f64, t4075: f64, t786: f64) -> (f64, f64, f64, f64, f64) {
    let t47520 = t2453 * t10174;
    let t47521 = t47520 * t9676;
    let t47525 = t3915 * t123 * t2434 * t4131;
    let t47527 = t10175 * t9686;
    let t47530 = t786 * t1420 * t4075;
    (t47520, t47521, t47525, t47527, t47530)
}
