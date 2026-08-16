//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1567/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1567(t3603: f64, t6622: f64, t1284: f64, t24698: f64, t487: f64, t83107: f64, t22648: f64, t602: f64, t1469: f64, t1486: f64, t72: f64, t23042: f64, t3915: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t84645 = t3603 * t6622;
    let t84859 = t24698 * t1284;
    let t84952 = t24698 * t487;
    let t84967 = t83107 * t487;
    let t85037 = t22648 * t602;
    let t85161 = t1469 * t1486 * t72;
    let t85475 = t3915 * t23042 * t72 * t686;
    (t84645, t84859, t84952, t84967, t85037, t85161, t85475)
}
