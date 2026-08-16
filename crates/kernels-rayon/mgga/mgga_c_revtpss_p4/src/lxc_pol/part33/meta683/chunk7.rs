//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2247/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2247(t29037: f64, t5378: f64, t20786: f64, t26849: f64, t29010: f64, t5265: f64, t20819: f64, t7617: f64, t104696: f64, t104793: f64, t104815: f64, t104817: f64, t104825: f64, t104828: f64, t104833: f64, t1252: f64, t20797: f64, t21046: f64, t97261: f64) -> f64 {
    let t112328 = t29037 * t5378;
    let t112334 = t26849 * t20786;
    let t112336 = t29010 * t5265;
    let t112339 = t20819 * t7617;
    let t112342 = -t104793 - 0.38110238327173099531e-3_f64 * t112328 + 0.42874018118069736972e-3_f64 * t104696 * t21046 + 0.42874018118069736972e-3_f64 * t97261 * t20797 - 0.28582678745379824648e-3_f64 * t112334 + 0.57165357490759649296e-3_f64 * t112336 - t104815 - t104817 + 0.19055119163586549765e-3_f64 * t104825 + t104828 + 0.42874018118069736972e-3_f64 * t112339 * t1252 + t104833;
    t112342
}
