//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2611/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2611(t1437: f64, t2482: f64, t4104: f64, t5658: f64, t2782: f64, t4086: f64, t48015: f64, t543: f64, t1882: f64, t3923: f64, t4003: f64, t10022: f64) -> (f64, f64, f64, f64) {
    let t48058 = t2482 * t1437 * t5658 * t4104;
    let t48066 = t2782 * t4086 * t48015 * t543;
    let t48073 = t1882 * t3923;
    let t48074 = t48073 * t4003;
    let t48076 = t2782 * t10022 * t48074;
    (t48058, t48066, t48073, t48076)
}
