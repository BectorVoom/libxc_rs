//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1774/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1774(t56236: f64, t58153: f64, t68399: f64, t68583: f64, t68585: f64, t68590: f64, t81236: f64, t81491: f64, t81496: f64, t81539: f64, t90486: f64, t90488: f64, t90490: f64, t90492: f64) -> f64 {
    let t90732 = -0.68863333333333333332e0_f64 * t81236 - 0.21424148148148148148e1_f64 * t56236 + 0.27545333333333333333e1_f64 * t68399 - 0.166712e1_f64 * t81491 - 0.12349037037037037037e0_f64 * t81496 - 0.12349037037037037037e1_f64 * t58153 + 0.27785333333333333333e0_f64 * t81539 - 0.705945e1_f64 * t90486 + 0.158837625e2_f64 * t90488 - 0.94674375e0_f64 * t90490 + 0.1262325e1_f64 * t90492 + 0.69463333333333333334e0_f64 * t68583 + 0.13892666666666666667e1_f64 * t68585 - 0.23154444444444444445e0_f64 * t68590;
    t90732
}
