//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 561/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk561(t1559: f64, t828: f64, t827: f64, t1544: f64, t855: f64, t1549: f64, t797: f64, t799: f64, t812: f64, t819: f64, t825: f64, t848: f64, t851: f64) -> (f64, f64, f64) {
    let t1560 = t828 * t1559;
    let t1561 = t827 * t1560;
    let t1565 = t855 * t828 * t1544;
    let t1568 = -t797 - t799 * t1549 / 48.0_f64 - t812 + t819 - 0.21437009059034868486e-3_f64 * t825 * t1561 - t848 - 0.85748036236139473944e-3_f64 * t851 * t1565;
    (t1561, t1565, t1568)
}
