//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2920/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2920(t2453: f64, t3908: f64, t4067: f64, t10115: f64, t1421: f64, t10168: f64, t3920: f64, t10174: f64, t9676: f64, t123: f64, t2434: f64, t3915: f64, t4131: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47510 = t2453 * t4067 * t3908;
    let t47512 = t10115 * t1421;
    let t47516 = t10168 * t3920;
    let t47520 = t2453 * t10174;
    let t47521 = t47520 * t9676;
    let t47525 = t3915 * t123 * t2434 * t4131;
    (t47510, t47512, t47516, t47520, t47521, t47525)
}
