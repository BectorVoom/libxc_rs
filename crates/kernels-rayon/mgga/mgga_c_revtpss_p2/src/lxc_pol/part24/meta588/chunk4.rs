//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1840/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1840(t22466: f64, t22852: f64, t39483: f64, t39520: f64, t39528: f64, t39531: f64, t4139: f64, t46963: f64, t46970: f64, t46972: f64, t5532: f64, t5536: f64, t6816: f64, t91956: f64, t91958: f64, t91959: f64, t91960: f64, t91961: f64, t91962: f64) -> f64 {
    let t92453 = -18.0_f64 * t22466 * t4139 * t6816 + 72.0_f64 * t22852 * t5532 * t5536 - t39483 + t39520 - t39528 + t39531 - t46963 + t46970 - t46972 + t91956 + t91958 - t91959 - t91960 + t91961 - t91962;
    t92453
}
