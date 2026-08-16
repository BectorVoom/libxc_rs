//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 816/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk816(t1907: f64, t615: f64, t1745: f64, t589: f64, t7312: f64, t8478: f64, t8492: f64, t8529: f64, t8558: f64, t8572: f64, t8578: f64, t9176: f64, t9178: f64, t9186: f64, t9190: f64, t9191: f64, t9196: f64, t9198: f64, t9199: f64, t9202: f64, t9206: f64, t9211: f64) -> (f64, f64) {
    let t9517 = t615 * t1907;
    let t9522 = t589 * t1745;
    let t9528 = t9176 + t7312 - t9178 + 0.62896184579208304136e-3_f64 * t8478 + 0.62896184579208304135e-3_f64 * t8492 - t9186 - t9190 + t9191 + 0.85748036236139473944e-3_f64 * t9522 - t9196 + 0.31448092289604152068e-3_f64 * t8529 + t9198 - t9199 - t9202 + t9206 - 0.62896184579208304136e-3_f64 * t8558 - 0.41930789719472202757e-3_f64 * t8572 - t9211 + 0.94344276868812456204e-3_f64 * t8578;
    (t9517, t9528)
}
