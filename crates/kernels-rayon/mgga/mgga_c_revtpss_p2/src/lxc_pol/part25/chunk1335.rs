//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1335/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1335(t7284: f64, t94810: f64, t25878: f64, t94597: f64, t10073: f64, t25937: f64, t7274: f64, t7282: f64, t1955: f64, t9656: f64, t1398: f64, t4077: f64, t543: f64) -> (f64, f64, f64, f64, f64) {
    let t94811 = t7284 * t94810;
    let t94813 = t25878 * t94597;
    let t94820 = t10073 * t7282 * t25937 * t7274;
    let t94823 = t1955 * t7282 * t9656;
    let t94825 = t4077 * t1398 * t543;
    (t94811, t94813, t94820, t94823, t94825)
}
