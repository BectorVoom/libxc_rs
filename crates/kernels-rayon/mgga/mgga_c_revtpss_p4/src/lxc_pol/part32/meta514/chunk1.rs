//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1815/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1815(t1312: f64, t1518: f64, t18245: f64, t2055: f64, t28653: f64, t30138: f64, t30143: f64, t30553: f64, t30570: f64, t30589: f64, t4248: f64, t5920: f64, t7359: f64, t7889: f64, t7983: f64) -> f64 {
    let t30612 = 2.0_f64 * t1312 * t30570 + 4.0_f64 * t1518 * t28653 + 2.0_f64 * t18245 * t2055 + 4.0_f64 * t2055 * t30138 + 2.0_f64 * t2055 * t30143 + 4.0_f64 * t4248 * t7983 + 2.0_f64 * t5920 * t7359 + 4.0_f64 * t7889 * t7983 + t30553 + 2.0_f64 * t30589;
    t30612
}
