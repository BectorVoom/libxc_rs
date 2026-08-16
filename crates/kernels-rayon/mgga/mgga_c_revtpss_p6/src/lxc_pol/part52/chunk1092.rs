//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1092/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1092(t4248: f64, t8641: f64, t7732: f64, t1936: f64, t8065: f64, t651: f64, t7898: f64, t8715: f64, t1955: f64, t8085: f64, t28911: f64, t7925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34193 = 2.0_f64 * t4248 * t8641;
    let t34195 = 2.0_f64 * t7732 * t8641;
    let t34196 = t8065 * t1936;
    let t34198 = 2.0_f64 * t651 * t34196;
    let t34203 = t7898 * t8715;
    let t34204 = t1955 * t8085;
    let t34212 = t28911 * t7925;
    (t34193, t34195, t34196, t34198, t34203, t34204, t34212)
}
