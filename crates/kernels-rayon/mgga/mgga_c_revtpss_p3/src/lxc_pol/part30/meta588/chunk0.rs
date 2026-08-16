//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2046/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2046(t136: f64, t2457: f64, t7307: f64, t25944: f64, t26035: f64, t686: f64, t72: f64, t7284: f64, t25878: f64, t94597: f64, t10073: f64, t25937: f64, t7274: f64, t7282: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94806 = t7307 * t136 * t2457;
    let t94807 = t25944 * t94806;
    let t94810 = t26035 * t72 * t686;
    let t94811 = t7284 * t94810;
    let t94813 = t25878 * t94597;
    let t94820 = t10073 * t7282 * t25937 * t7274;
    (t94806, t94807, t94810, t94811, t94813, t94820)
}
