//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1697/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1697(t88432: f64, t88445: f64, t88448: f64, t88451: f64, t88481: f64, t88580: f64, t88584: f64, t88586: f64, t88588: f64, t88590: f64, t88592: f64, t88596: f64) -> f64 {
    let t88983 = t88432 - t88580 + t88584 - t88445 + t88448 + t88451 - t88586 - t88588 + t88481 + t88590 - t88592 - t88596;
    t88983
}
