//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2241/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2241(t3655: f64, t8185: f64, t17628: f64, t7607: f64, t104943: f64, t17280: f64, t17651: f64, t17800: f64, t1791: f64, t26827: f64, t5320: f64, t7613: f64, t97174: f64, t97279: f64, t97281: f64, t97283: f64, t97288: f64, t97296: f64) -> f64 {
    let t104988 = t8185 * t3655;
    let t104990 = t7607 * t17628;
    let t104992 = -0.42874018118069736972e-3_f64 * t97283 * t1791 - 0.85748036236139473944e-3_f64 * t26827 * t5320 - 0.42874018118069736972e-3_f64 * t7613 * t17280 + 0.57165357490759649296e-3_f64 * t97279 - 0.28582678745379824648e-3_f64 * t97281 + 0.19055119163586549765e-3_f64 * t97288 + t97296 - 0.11433071498151929859e-2_f64 * t104943 * t17800 + 0.57165357490759649296e-3_f64 * t97174 * t17651 + 0.5081365110289746604e-3_f64 * t104988 + t104990 / 1296.0_f64;
    t104992
}
