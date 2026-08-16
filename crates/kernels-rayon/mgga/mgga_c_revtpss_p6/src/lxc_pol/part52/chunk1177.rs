//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1177/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1177(t125922: f64, t32265: f64, t32269: f64, t125849: f64, t552: f64, t8590: f64, t1405: f64, t33959: f64, t1448: f64, t7933: f64, t27123: f64, t8461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125923 = t32265 * t125922;
    let t125925 = t32269 * t125922;
    let t125928 = t125849 * t8590 * t552;
    let t125930 = t33959 * t1405;
    let t125939 = t7933 * t1448;
    let t125948 = 2.0_f64 * t27123 * t8461;
    (t125923, t125925, t125928, t125930, t125939, t125948)
}
