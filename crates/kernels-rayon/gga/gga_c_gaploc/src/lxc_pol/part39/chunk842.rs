//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 842/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk842(t107: f64, t33137: f64, t787: f64, t10012: f64, t10627: f64, t10892: f64, t1980: f64, t1858: f64, t3431: f64, t2101: f64, t1890: f64, t3487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33139 = t787 * t33137 * t107;
    let t33148 = t10012 * t10627;
    let t33206 = t1980 * t10892;
    let t33232 = t1858 * t3431;
    let t33285 = t2101 * t3431;
    let t33289 = t1890 * t3487;
    (t33139, t33148, t33206, t33232, t33285, t33289)
}
