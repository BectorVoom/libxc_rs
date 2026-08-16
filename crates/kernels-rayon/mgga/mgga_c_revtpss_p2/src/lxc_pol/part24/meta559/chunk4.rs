//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1682/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1682(t300: f64, t88077: f64, t88364: f64, t88499: f64, t88570: f64, t5023: f64, t63907: f64, t6400: f64, t88046: f64, t88048: f64, t88050: f64, t88052: f64, t88054: f64, t88140: f64, t88358: f64, t88361: f64, t88363: f64, t88368: f64, t88432: f64) -> (f64, f64) {
    let t88573 = t300 * (t88077 + t88364 + t88499 + t88570);
    let t88577 = 12.0_f64 * t5023 * t63907 * t6400 - t88046 + t88048 + t88050 + t88052 + t88054 - t88140 + t88358 - t88361 + t88363 - t88368 + t88432 + t88573;
    (t88573, t88577)
}
