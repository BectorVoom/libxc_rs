//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3105/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3105(t58226: f64, t68454: f64, t68456: f64, t68538: f64, t68540: f64, t68548: f64, t68550: f64, t68567: f64, t68583: f64, t68585: f64, t68590: f64, t81539: f64) -> f64 {
    let t81766 = 0.69463333333333333333e-1_f64 * t81539 - 0.83356000000000000002e0_f64 * t68538 - 0.125034e1_f64 * t68540 + 0.13892666666666666667e0_f64 * t68548 + 0.41678000000000000001e0_f64 * t68550 - 0.20659e1_f64 * t68454 - 0.309885e1_f64 * t68456 - 0.20839e0_f64 * t68567 + t58226 + 0.34731666666666666667e0_f64 * t68583 + 0.69463333333333333335e0_f64 * t68585 - 0.11577222222222222223e0_f64 * t68590;
    t81766
}
