//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2035/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2035(t11874: f64, t27492: f64, t11988: f64, t7132: f64, t3196: f64, t7131: f64, t11648: f64, t7122: f64, t25512: f64, t3173: f64, t11916: f64, t25509: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93548 = t11874 * t27492;
    let t93555 = t7132 * t11988;
    let t93561 = t3196 * t7131;
    let t93564 = t7122 * t11648;
    let t93570 = t25512 * t3173;
    let t93573 = t25509 * t11916;
    (t93548, t93555, t93561, t93564, t93570, t93573)
}
