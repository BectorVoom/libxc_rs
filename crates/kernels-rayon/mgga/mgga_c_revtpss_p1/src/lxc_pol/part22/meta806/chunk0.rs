//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2908/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2908(t3863: f64, t4029: f64, t1340: f64, t40135: f64, t4038: f64, t9425: f64, t1330: f64, t512: f64, t9544: f64, t3869: f64, t39739: f64, t39430: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47101 = t3863 * t4029;
    let t47109 = 0.6233709278045326953e3_f64 * t1340 * t40135;
    let t47110 = t4038 * t9425;
    let t47113 = t512 * t1330 * t9544;
    let t47116 = 0.86748650402413918736e-1_f64 * t3869 * t39739;
    let t47118 = 0.38527786510141256862e1_f64 * t3869 * t39430;
    (t47101, t47109, t47110, t47113, t47116, t47118)
}
