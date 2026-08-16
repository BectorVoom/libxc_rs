//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1493/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1493(t14220: f64, t48007: f64, t22331: f64, t2470: f64, t4101: f64, t10073: f64, t22369: f64, t136: f64, t2457: f64, t47429: f64, t6862: f64, t22351: f64, t2439: f64, t2777: f64) -> (f64, f64, f64, f64, f64) {
    let t75005 = t48007 * t14220;
    let t75021 = t4101 * t22331 * t2470;
    let t75026 = t10073 * t22369;
    let t75068 = t47429 * t6862 * t136 * t2457;
    let t75074 = t2439 * t2777 * t22351;
    (t75005, t75021, t75026, t75068, t75074)
}
