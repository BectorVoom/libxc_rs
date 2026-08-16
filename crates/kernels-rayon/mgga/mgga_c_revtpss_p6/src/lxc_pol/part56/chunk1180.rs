//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1180/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1180(t33515: f64, t33516: f64, t5291: f64, t12915: f64, t247: f64, t33405: f64, t34934: f64, t3736: f64, t482: f64, t31993: f64, t33524: f64, t5377: f64) -> (f64, f64, f64, f64) {
    let t131556 = t33515 * t33516 * t5291;
    let t131576 = t33405 * t247 * t12915 * t34934;
    let t131578 = t482 * t3736;
    let t131584 = t33524 * t31993 * t5377;
    (t131556, t131576, t131578, t131584)
}
