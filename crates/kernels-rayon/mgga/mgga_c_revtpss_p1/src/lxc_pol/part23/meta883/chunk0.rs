//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2794/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2794(t22351: f64, t2439: f64, t2777: f64, t22253: f64, t4101: f64, t686: f64, t72: f64, t22335: f64, t2470: f64, t10073: f64, t22361: f64, t10069: f64, t22373: f64) -> (f64, f64, f64, f64, f64) {
    let t75074 = t2439 * t2777 * t22351;
    let t75089 = t4101 * t22253 * t72 * t686;
    let t75092 = t4101 * t22335 * t2470;
    let t75113 = t10073 * t22361;
    let t75119 = t10069 * t22373;
    (t75074, t75089, t75092, t75113, t75119)
}
