//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2791/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2791(t1883: f64, t5658: f64, t2782: f64, t4100: f64, t543: f64, t73842: f64, t22331: f64, t2470: f64, t4101: f64, t48048: f64, t5741: f64, t10073: f64, t22369: f64) -> (f64, f64, f64, f64, f64) {
    let t75012 = t1883 * t5658;
    let t75014 = t2782 * t4100 * t75012;
    let t75016 = t73842 * t543;
    let t75018 = t2782 * t4100 * t75016;
    let t75021 = t4101 * t22331 * t2470;
    let t75024 = t48048 * t5741;
    let t75026 = t10073 * t22369;
    (t75014, t75018, t75021, t75024, t75026)
}
