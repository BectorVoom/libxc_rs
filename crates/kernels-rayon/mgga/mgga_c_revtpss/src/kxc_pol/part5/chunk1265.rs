//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1265/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1265(t19508: f64, t19554: f64, t19606: f64, t20149: f64, t1079: f64, t20112: f64, t225: f64, t385: f64, t1096: f64, t6392: f64, t3269: f64, t1647: f64, t1678: f64) -> (f64, f64, f64, f64) {
    let t20151 = t19508 + t19554 + t19606 + t20149;
    let t20152 = t1079 * t20151;
    let t20168 = t20112 * t225 * t385;
    let t20171 = t6392 * t1096;
    let t20172 = t3269 * t20171;
    let t20175 = t1647 * t1678;
    (t20152, t20168, t20172, t20175)
}
