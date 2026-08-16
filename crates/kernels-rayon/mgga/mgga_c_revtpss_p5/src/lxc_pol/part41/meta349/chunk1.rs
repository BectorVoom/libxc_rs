//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1158/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1158(t12046: f64, t378: f64, t342: f64, t1647: f64, t3316: f64, t1071: f64, t4746: f64, t15669: f64, t379: f64, t994: f64, t1716: f64, t2435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16565 = t12046 * t378;
    let t16566 = t342 * t16565;
    let t16584 = t1647 * t3316;
    let t16597 = t4746 * t1071;
    let t16600 = t15669 * t378;
    let t16603 = t994 * t379;
    let t16706 = t2435 * t1716;
    (t16566, t16584, t16597, t16600, t16603, t16706)
}
