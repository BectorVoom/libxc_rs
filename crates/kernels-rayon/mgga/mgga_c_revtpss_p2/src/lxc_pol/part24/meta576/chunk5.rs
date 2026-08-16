//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1767/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1767(t90509: f64, t90511: f64, t90514: f64, t90578: f64, t90580: f64, t90582: f64, t90585: f64, t90588: f64, t90592: f64, t90594: f64, t90597: f64, t90599: f64) -> f64 {
    let t90600 = t90509 + t90511 - t90514 + t90578 - t90580 - t90582 + t90585 + t90588 + t90592 - t90594 - t90597 + t90599;
    t90600
}
