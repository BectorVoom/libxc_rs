//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1130/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1130(t119823: f64, t126129: f64, t119875: f64, t33682: f64, t31837: f64, t33695: f64, t31841: f64, t33687: f64, t686: f64, t72: f64, t120140: f64, t31838: f64, t33715: f64, t845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t126208 = t119823 * t126129;
    let t126210 = t119875 * t33682;
    let t126213 = t33695 * t31837;
    let t126214 = t126213 * t31841;
    let t126221 = t33687 * t72 * t686;
    let t126222 = t120140 * t126221;
    let t126226 = t31838 * t845 * t33715;
    (t126208, t126210, t126214, t126221, t126222, t126226)
}
